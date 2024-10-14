
function setup() {
    cd src/rust_hello_backend
    dfx start --clean --background # canister-http default on
}

function teardown() {
    dfx stop
}

@test "init token" {
    dfx deploy
    run dfx canister call rust_hello_backend init_token '( "TEST", "Test Token", 1000000, 8 )'
    [ "$status" -eq 0 ]
}

@test "check balance" {
    dfx deploy
    # owner_principal=$(dfx identity get-principal)
    run dfx canister call rust_hello_backend balance_of '(principal "5efnn-telku-74vvo-k7d22-adlyf-p4kkz-ehcns-3dxsp-s5x5m-okk6e-yae")'   
    [ "$status" -eq 0 ]
 
}

@test "check transfer" {
    # Transfer some tokens from the owner to another principal
    # owner_principal=$(dfx identity get-principal)
    # receiver_principal="aaaaa-aa"  # Replace with a valid principal or generate a new identity
    dfx deploy
    run dfx canister call rust_hello_backend transfer '(principal "pslmo-vbi5f-lrz6s-m5ocl-a75r3-7vszq-kn64o-4ucje-hfud5-aeiaf-kae", 50000)'
    [ "$status" -eq 0 ]


    # Check that the balance is updated correctly for both the owner and receiver
    run dfx canister call rust_hello_backend balance_of '(principal "5efnn-telku-74vvo-k7d22-adlyf-p4kkz-ehcns-3dxsp-s5x5m-okk6e-yae")'   
    [ "$status" -eq 0 ]


    run dfx canister call rust_hello_backend balance_of '(principal "pslmo-vbi5f-lrz6s-m5ocl-a75r3-7vszq-kn64o-4ucje-hfud5-aeiaf-kae")'
    [ "$status" -eq 0 ]
 
}