package main

import (
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
    	w.Header().Set("Content-Type", "application/json")
    	if _, err := w.Write([]byte(`{"status":"ok"}`)); err != nil {
        	log.Printf("failed to write health response: %v", err)
        	http.Error(w, "failed to write response", http.StatusInternalServerError)
        	return
    	}
	})

	addr := ":8080"
	log.Printf("switchapi: listening on %s\n", addr)
	if err := http.ListenAndServe(addr, nil); err != nil {
		log.Fatalf("server failed: %v", err)
	}
}
