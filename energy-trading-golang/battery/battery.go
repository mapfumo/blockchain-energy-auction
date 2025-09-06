package main

import (
	"flag"
	"fmt"
	"log"
	"net"

	"github.com/energy-trading/message"
)

type battery struct {
	id                int
	energyTotal       float64 // kWh
	percentageForSale float64
	reservePrice      float64 // cents/kWh
	highestBid        float64 // cents/kWh
	aggregatorID      int     // aggregator ID
}

func main() {
	var batteryID int
	var energyTotal float64
	var percentageForSale float64
	var reservePrice float64
	var addr string
	var network string

	flag.IntVar(&batteryID, "id", 50, "Battery/BESS ID")
	flag.Float64Var(&energyTotal, "t", 13.5,
		"Total energy in battery (cents/kWh)")
	flag.Float64Var(&percentageForSale, "p", 50.0,
		"Percentage of available energy for sale")
	flag.Float64Var(&reservePrice, "reserveprice", 14.0, "Minimum selling price")
	flag.StringVar(&addr, "e", ":4000",
		"service endpoint [ip addr or socket path]")
	flag.StringVar(&network, "n", "tcp", "network protocol [tcp,unix")
	flag.Parse()

	b := newBattery(batteryID, energyTotal, percentageForSale, reservePrice)
	listener, err := net.Listen(network, addr)
	if err != nil {
		log.Fatal("failed to create listener:", err)
	}
	defer listener.Close()
	b.print()
	log.Printf("Service started: (%s) %s\n", network, addr)

	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println(err)
			if err := conn.Close(); err != nil {
				log.Println("failed to close listener:", err)
			}
			continue
		}
		log.Println("Connected to", conn.RemoteAddr())

		go handleConnection(conn, &b)
	}
}

func handleConnection(conn net.Conn, b *battery) {
	defer func() {
		if err := conn.Close(); err != nil {
			log.Println("error closing connection", err)
		}
	}()
	for {
		bs := make([]byte, 1024)
		n, err := conn.Read(bs)
		if err != nil {
			break
		}
		msg := message.FromNetwork(string(bs[:n]))
		switch msg.GetMessageType() {
		case 0: // device register
			fmt.Println("received REGISTER message:", msg.Serialize())
		case 1: // Query Message
			fmt.Println("received QUERY message:", msg.Serialize())
			queryResponse(conn, b)
		case 3: // bid/offer message
			fmt.Println("received BID message:", msg.Serialize())
			bidResponse(msg, conn, b)
		case 5:
			fmt.Println("received BID CONFIRMED message:", msg.Serialize())
			b.setPercentageForSale(b.getPercentageForSale() - msg.GetPercentageForSale())
			fmt.Println("Percentage for sale left =", b.getPercentageForSale())
		default:
			fmt.Println("Unknown message type....")
		}

	}
}

func queryResponse(conn net.Conn, b *battery) {
	msg := message.NewMessage(2, b.id)
	msg.SetEnergyTotal(b.energyTotal)
	msg.SetPercentageForSale(b.percentageForSale)
	msg.Send(conn)
}

func bidResponse(msg message.Message, conn net.Conn, b *battery) {
	if msg.GetBidPrice() < b.GetReservePrice() {
		fmt.Println("Bid too low, REJECTED:", msg.GetBidPrice(), "cents/kWh")
		msg.SetMessageType(6) // reject message type
		msg.SetDeviceID(b.id)
		msg.Send(conn)
	} else { // accept bid
		fmt.Println("Bid ACCEPED:", msg.GetBidPrice(), "cents/kWh, Highest bider ID:", msg.GetDeviceID())
		msg.SetMessageType(4)
		msg.SetDeviceID(b.id)
		msg.Send(conn)
	}
}

func newBattery(batteryID int, energyTotal float64,
	percentageForSale float64, reservePrice float64) battery {
	b := battery{
		id:                batteryID,
		energyTotal:       energyTotal,
		percentageForSale: percentageForSale,
		reservePrice:      reservePrice,
	}
	return b
}

// GetDeviceID get this device ID
func (b *battery) GetDevideID() int {
	return b.id
}

func (b battery) getEnergyTotal() float64 {
	return b.energyTotal
}

func (b battery) getPercentageForSale() float64 {
	return b.percentageForSale
}

// GetReservePrice get the reserve price
func (b battery) GetReservePrice() float64 {
	return b.reservePrice
}

func (b battery) getHighestBid() float64 {
	return b.highestBid
}

func (b battery) getAggregatorID() int {
	return b.aggregatorID
}

func (b *battery) setDevideID(newID int) {
	b.id = newID
}

func (b *battery) setEnergyTotal(newEnergyTotal float64) {
	b.energyTotal = newEnergyTotal
}

func (b *battery) setPercentageForSale(newPercentageForSale float64) {
	b.percentageForSale = newPercentageForSale
}

func (b *battery) setReservePrice(newReservePrice float64) {
	b.reservePrice = newReservePrice
}

func (b *battery) setHighestBid(newHighestBid float64, newAggregatorID int) bool {
	if newHighestBid < b.highestBid {
		fmt.Println("Too low, bid rejected")
		return false
	}
	b.highestBid = newHighestBid
	b.aggregatorID = newAggregatorID
	return true
}

func (b battery) print() {
	fmt.Println("BESS ID:", b.id, "| Total Energy:",
		b.energyTotal, "kWh |", b.percentageForSale, "% is for sale | reserve price:", b.reservePrice, "cents/kWh)")
	fmt.Println("Highest bid:", b.highestBid, "cents/kWh | by Aggregator ID:", b.aggregatorID)
}
