# NOTES

## TLDR MD Notes

- `'Rust - ActixWeb v4.0 - Add React Project'`

## TLDR Debug

just stop service and launch `Debug executable 'actixweb4-starter'`

## TLDR Run Project

```shell
# win1 server
$ make startConfigServer
# or
# make startConfig
```

1. test frontend <https://localhost:8543/>
2. test health check `curl -s -k --request GET --url https://localhost:8543/ping --header 'content-type: application/json' | jq`
3. test protected endpoint `curl -s -k --request GET --url https://localhost:8543/api/state --header 'authorization: Bearer uOtXEZXYslKyB0n3g3xRmCaaNsAwB5KmgFcy1X7bbcbtS9dhOpKuhZ04Mfr2OKGL' --header 'content-type: application/json' | jq`

```shell
# win2 actixweb web client
$ make startClient
```

goto <https://localhost:8545/>

> require node v16.13.2

add breakpoint at

```typescript
function App() {
  console.log('breakpoint');
  return (
```

now launch debug `"Debug React Frontend"` or press `F5`

test websockets open console and launch `curl curl -s -k --request POST --url https://localhost:8543/api/ws-echo --header 'authorization: Bearer uOtXEZXYslKyB0n3g3xRmCaaNsAwB5KmgFcy1X7bbcbtS9dhOpKuhZ04Mfr2OKGL' --header 'content-type: application/json' --data '{"message": "hi there"}' | jq`
