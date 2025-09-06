package message

import (
	"fmt"
	"math/rand"
	"net"
	"strconv"
	"strings"
	"time"
)

// Message energy trading message
type Message struct {
	messageType             int     //register, query, etc
	messageID               int     // is also the the timestamp
	deviceID                int     // who generated this Message
	ttl                     int     // time to live
	bidPrice                float64 // offer/bid price, cents/kWh
	salePrice               float64 // final price, cents/kWh
	energyTotal             float64 // battery total energy available
	percentageForSale       float64 // %age of total available for sale
	requiredEnergyAmount    float64 // usually all of it = energy total
	terminationCode         int     //why was the transaction terminated
	remainingBatteryEnergy  float64
	batteryHealthStatusCode int
	batteryVoltage          float64
	dischargeRate           float64
}

//Serialize convert this message to string
func (m Message) Serialize() string {
	return fmt.Sprintf("%d %d %d %d %0.2f %0.2f %0.2f %0.2f %0.2f %d %0.2f %d %0.2f %0.2f",
		m.messageType, m.messageID, m.deviceID, m.ttl,
		m.bidPrice, m.salePrice, m.energyTotal, m.percentageForSale,
		m.requiredEnergyAmount, m.terminationCode, m.remainingBatteryEnergy,
		m.batteryHealthStatusCode, m.batteryVoltage, m.dischargeRate)
}

// Print print this message to stdio
func (m Message) Print() {
	fmt.Println(m.Serialize())
}

// FromNetwork reconstruct the message received over the network
func FromNetwork(bs string) Message {
	msg := strings.Split(bs, " ")
	return Message{
		messageType:             stringToInt(msg[0]),
		messageID:               stringToInt(msg[1]),
		deviceID:                stringToInt(msg[2]),
		ttl:                     stringToInt(msg[3]),
		bidPrice:                stringToFloat(msg[4]),
		salePrice:               stringToFloat(msg[5]),
		energyTotal:             stringToFloat(msg[6]),
		percentageForSale:       stringToFloat(msg[7]),
		requiredEnergyAmount:    stringToFloat(msg[8]),
		terminationCode:         stringToInt(msg[9]),
		remainingBatteryEnergy:  stringToFloat(msg[10]),
		batteryHealthStatusCode: stringToInt(msg[11]),
		batteryVoltage:          stringToFloat(msg[12]),
		dischargeRate:           stringToFloat(msg[13]),
	}
	//return Message{}
}

func stringToInt(str string) int {
	n, err := strconv.ParseInt(str, 10, 64)
	if err != nil {
		return -1
	}
	return int(n)
}

func stringToFloat(str string) float64 {
	n, err := strconv.ParseFloat(str, 64)
	if err != nil {
		return -1.0
	}

	return n
}

// NewMessage create new message type
func NewMessage(newMessageType int, newDeviceID int) Message {
	rand.Seed(time.Now().UnixNano())
	return Message{
		messageType: newMessageType,
		messageID:   (10000 + rand.Intn(99999-10000)),
		deviceID:    newDeviceID,
	}
}

// QueryMessage create a query message to send to battery
func QueryMessage(newDeviceID int) Message {
	rand.Seed(time.Now().UnixNano())
	return Message{
		messageType: 1,
		messageID:   (10000 + rand.Intn(99999-10000)),
		deviceID:    newDeviceID,
	}
}

// Send sends the message over the connection
func (m Message) Send(conn net.Conn) {
	conn.Write([]byte(m.Serialize()))
}

// GetMessageType returns the message type 0: Register, 1: Query, ...
func (m Message) GetMessageType() int {
	return m.messageType
}

func getMessageID(m Message) int {
	return m.messageID
}

// GetDeviceID get the device id
func (m Message) GetDeviceID() int {
	return m.deviceID
}

func getTTL(m Message) int {
	return m.ttl
}

// GetBidPrice get current bit
func (m Message) GetBidPrice() float64 {
	return m.bidPrice
}

func getSalePrice(m Message) float64 {
	return m.salePrice
}

// GetEnergyTotal get energy total
func (m Message) GetEnergyTotal() float64 {
	return m.energyTotal
}

// GetPercentageForSale what percentage of the available energy is for sale
func (m Message) GetPercentageForSale() float64 {
	return m.percentageForSale
}
func getRequiredEnergyAmount(m Message) float64 {
	return m.requiredEnergyAmount
}

func getTerminationCode(m Message) int {
	return m.terminationCode
}

func getRemainingBatteryEnergy(m Message) float64 {
	return m.remainingBatteryEnergy
}

func getBatteryHealthStatusCode(m Message) int {
	return m.batteryHealthStatusCode
}

func getBatteryVoltage(m Message) float64 {
	return m.batteryVoltage
}

func getDischargeRate(m Message) float64 {
	return m.dischargeRate
}

// ===================Receivers =====================================//

//SetMessageType set new message type
func (m *Message) SetMessageType(newMessageType int) {
	m.messageType = newMessageType
}

func (m *Message) setMessageID(newMessageID int) {
	m.messageID = newMessageID
}

//SetDeviceID set device ID
func (m *Message) SetDeviceID(newDeviceID int) {
	m.deviceID = newDeviceID
}

func (m *Message) setTTL(newTTL int) {
	m.ttl = newTTL
}

//SetBidPrice set bidding price
func (m *Message) SetBidPrice(newBidPrice float64) {
	m.bidPrice = newBidPrice
}

func (m *Message) setSalePrice(newSalePrice float64) {
	m.salePrice = newSalePrice
}

//SetEnergyTotal (newEnergyTotal float64)
func (m *Message) SetEnergyTotal(newEnergyTotal float64) {
	m.energyTotal = newEnergyTotal
}

//SetPercentageForSale (newPercentageForSale float64)
func (m *Message) SetPercentageForSale(newPercentageForSale float64) {
	m.percentageForSale = newPercentageForSale
}
func (m *Message) setRequiredEnergyAmount(newEnergyAmount float64) {
	m.requiredEnergyAmount = newEnergyAmount
}

func (m *Message) setTerminationCode(newTerminationCode int) {
	m.terminationCode = newTerminationCode
}

func (m *Message) setRemainingBatteryEnergy(newRemainingBatteryEnergy float64) {
	m.remainingBatteryEnergy = newRemainingBatteryEnergy
}

func (m *Message) setBatteryHealthStatusCode(newBatteryHealthStatusCode int) {
	m.batteryHealthStatusCode = newBatteryHealthStatusCode
}

func (m *Message) setBatteryVoltage(newBatteryVoltage float64) {
	m.batteryVoltage = newBatteryVoltage
}

func (m *Message) setDischargeRate(newDischargeRate float64) {
	m.dischargeRate = newDischargeRate
}
