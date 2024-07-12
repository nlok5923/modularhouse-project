package main

import (
	"da-server/avail"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
)

type Orders struct {
	SellOrders []int  `json:"sell_orders"`
	BuyOrders  []int  `json:"buy_orders"`
	Pair       string `json:"pair"`
}

func main() {

	file, err := os.Open("order.json")
	if err != nil {
		log.Fatalf("failed to open file: %s", err)
	}
	defer file.Close()

	// Read the file contents
	byteValue, err := ioutil.ReadAll(file)
	if err != nil {
		log.Fatalf("failed to read file: %s", err)
	}

	// Unmarshal the JSON into the struct
	var orders Orders
	err = json.Unmarshal(byteValue, &orders)
	if err != nil {
		log.Fatalf("failed to unmarshal JSON: %s", err)
	}

	// Marshal the struct back to a JSON string
	jsonString, err := json.Marshal(orders)
	if err != nil {
		log.Fatalf("failed to marshal JSON: %s", err)
	}

	fmt.Println("Order data: %v", string(jsonString))

	http.HandleFunc("/submit_data", func(w http.ResponseWriter, r *http.Request) {
		avail.DataSubmit(10, "wss://turing-rpc.avail.so/ws", "bulk impact process private orange motion roof force clean recall filter secret", 0, string(jsonString))
		println("Data submitted to Avail")
	})

	fmt.Printf("Starting server at port 8080\n")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}
}
