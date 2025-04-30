package main

// Root represents the top-level JSON response from CoinMarketCap API
type Root struct {
	Status Status `json:"status"`
	Data   Data   `json:"data"`
}

// Status contains API response status information
type Status struct {
	Timestamp string `json:"timestamp"`
}

// Data contains cryptocurrency data
type Data struct {
	Symbol     string     `json:"symbol"`
	Statistics Statistics `json:"statistics"`
}

// Statistics contains price statistics
type Statistics struct {
	Price float64 `json:"price"`
}

// PriceFeedData is the output structure with essential price information
type PriceFeedData struct {
	Symbol    string  `json:"symbol"`
	Price     float64 `json:"price"`
	Timestamp string  `json:"timestamp"`
}
