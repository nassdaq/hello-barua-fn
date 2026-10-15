// The same function in Go. Deploy this repository with folder "go".
package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	data, _ := io.ReadAll(os.Stdin)
	name := strings.TrimSpace(string(data))
	var in struct {
		Name string `json:"name"`
	}
	if json.Unmarshal(data, &in) == nil && in.Name != "" {
		name = in.Name
	}
	if name == "" {
		name = "world"
	}
	greeting, fromEnv := os.LookupEnv("GREETING")
	if !fromEnv {
		greeting = "Hello"
	}
	out, _ := json.Marshal(map[string]any{
		"message":           fmt.Sprintf("%s, %s!", greeting, name),
		"lang":              "go",
		"greeting_from_env": fromEnv,
	})
	os.Stdout.Write(out)
}
