package main

import (
	"flag"
	"fmt"
	"log"
	"math/rand"
	"net"
	"time"

	"github.com/energy-trading/message"
)

// Aggregator data structure
type Aggregator struct {
	id int
}

var maxBidPrice float64

func main() {
	var aggregatorID int
	flag.IntVar(&aggregatorID, "id", 20, "Battery/BESS ID")
	flag.Float64Var(&maxBidPrice, "maxbidprice", 18.5, "Max bid price, cents/kWh")
	flag.Parse()
	aggregator := NewAggregator(aggregatorID)
	trade(aggregator)
}

// NewAggregator create new aggregator
func NewAggregator(newID int) Aggregator {
	return Aggregator{
		id: newID,
	}
}

func trade(aggregator Aggregator) {
	//servAddr := "192.168.1.110:4000
	servAddr := "127.0.0.1:4000"
	conn, err := net.Dial("tcp", servAddr)
	if err != nil {
		log.Fatal(err)
	}

	defer conn.Close()

	msg := message.QueryMessage(aggregator.id)
	log.Println("Sending query message:", msg.Serialize())
	msg.Send(conn)
	finishedTrading := false
	for {
		replyMsg := make([]byte, 1024)
		n, err := conn.Read(replyMsg)
		if err != nil {
			panic(err)
		}
		msg = message.FromNetwork(string(replyMsg[:n]))
		switch msg.GetMessageType() {
		case 2:
			log.Println("Received Query Response: ", msg.Serialize())
			fmt.Println(msg.GetPercentageForSale(), "% of", msg.GetEnergyTotal(), "kWh available for sale")
			if msg.GetPercentageForSale() > 0 {
				// bid for all of it at current price + 5 cents
				placeBid(msg, conn, aggregator)

			} else {
				log.Println("Nothing to buy, closing connection")
				return
			}
		case 4:
			log.Println("Bid ACCEPED:", msg.GetPercentageForSale()*msg.GetEnergyTotal()/100,
				"kWh @", msg.GetBidPrice(), "cents/kWh")
			confirmBid(msg, conn, aggregator)
			finishedTrading = true
			break
		case 6:
			log.Println("Bid REJECTED: ", msg.GetBidPrice(), "cents/kWh")
			placeBid(msg, conn, aggregator) //bid again
		default:
			fmt.Println("Unknown message type....")
		}
		if finishedTrading == true {
			log.Println("Transaction COMPLETED, Exiting...")
			break
		}
	}
}

func confirmBid(msg message.Message, conn net.Conn, a Aggregator) {
	msg.SetMessageType(5) // bid confirmation message Type 5
	msg.SetDeviceID(a.id)
	msg.Send(conn)
}

func placeBid(m message.Message, conn net.Conn, a Aggregator) {
	offer := float64(rand.Intn(5)) + 1 // increase with random offer between 1 - 10 cents/kWh
	if (offer + m.GetBidPrice()) <= maxBidPrice {
		m.SetBidPrice(offer + m.GetBidPrice())
		m.SetDeviceID(a.id) // set the aggregator's device ID
		log.Println("Placing new bid @ ", m.GetBidPrice(), "cents/kWh")
		m.SetMessageType(3)
		n := rand.Intn(3)
		time.Sleep(time.Duration(n) * time.Second) // wait a few random seconds
		m.Send(conn)
	} else {
		log.Println("MAX BID reached. Ending bidding...")
		return
	}

}

func queryBattery(aggregator Aggregator, conn net.Conn) {

}
