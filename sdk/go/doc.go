// Package overlayward provides a Go client SDK for the Overlayward Sandbox Manager API.
//
// Usage:
//
//	client, err := overlayward.NewClient(overlayward.Config{
//	    Endpoint: "localhost:8421",
//	    Token:    "ow-agent-token",
//	})
//	sandbox, err := client.Sandbox().Create(ctx, &overlayward.CreateOpts{Name: "dev"})
//
// Generate stubs: make gen-go
package overlayward
