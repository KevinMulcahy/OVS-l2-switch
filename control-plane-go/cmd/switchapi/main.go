package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

// healthResponse defines the JSON structure for /healthz responses.
type healthResponse struct {
	Status string `json:"status"`
}

func main() {
	log.Println("switchapi: listening on :8080")

	mux := http.NewServeMux()

	// Health endpoint for Docker/CI checks.
	mux.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		if err := json.NewEncoder(w).Encode(healthResponse{Status: "ok"}); err != nil {
			http.Error(w, "failed to encode health response", http.StatusInternalServerError)
			return
		}
	})

	// Basic root endpoint.
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		if _, err := w.Write([]byte("controlplane OK\n")); err != nil {
			log.Printf("error writing response: %v", err)
		}
	})

	srv := &http.Server{
		Addr:    ":8080",
		Handler: mux,
	}

	// Run server in background so we can capture shutdown signals.
	go func() {
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("server error: %v", err)
		}
	}()

	// Wait for termination signals.
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	log.Println("controlplane: shutting down gracefully...")
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		log.Fatalf("controlplane: forced to shutdown: %v", err)
	}

	log.Println("controlplane: stopped")
}