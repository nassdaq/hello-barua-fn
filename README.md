# hello-barua-fn

A function for [Barua Functions](https://fn.barua.app). Paste this repository's
URL there and it becomes `https://fn.barua.app/hello-barua-fn`.

```sh
curl -X POST https://fn.barua.app/hello-barua-fn --data '{"name": "Asha"}'
# {"message":"Hello, Asha!","lang":"rust","greeting_from_env":false}
```

Set `GREETING` in the function's environment on that page and the word changes.
The same function in Go lives in `go/` — deploy it with the folder set to `go`.

A function reads its input on stdin and prints its answer on stdout. That is
the whole contract.
