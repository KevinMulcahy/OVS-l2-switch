package main

import (
	"io"
	"net/http"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestHealthEndpoint(t *testing.T) {
	resp, err := http.Get("http://controlplane:8080/health")
	require.NoError(t, err, "failed to reach /health endpoint")
	defer resp.Body.Close()

	require.Equal(t, http.StatusOK, resp.StatusCode, "unexpected status code")

	body, err := io.ReadAll(resp.Body)
	require.NoError(t, err, "failed to read response body")
	require.JSONEq(t, `{"status":"ok"}`, string(body))
}
