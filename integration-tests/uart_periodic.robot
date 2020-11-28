*** Settings ***
Suite Setup     Setup
Suite Teardown  Teardown
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}

*** Keywords ***
Create Machine
    Execute Command         mach create
    Execute Command         machine LoadPlatformDescription @platforms/boards/stm32f4_discovery-kit.repl
    Execute Command         machine LoadPlatformDescription @${CURDIR}/../renode/add-ccm.repl
    Execute Command         sysbus LoadELF @${CURDIR}/../target/thumbv7em-none-eabihf/debug/examples/uart_timer
    Create Terminal Tester  sysbus.uart2
    Start Emulation

*** Test Cases ***
Verify Timing
    # Documentation Verifies timer settings are accurate
    # [Tags] timer interrupt uart

    # Start Test
    Create Machine
    Wait For Line On Uart   Task1 start: 0
    Wait For Line On Uart   Task1 end: 50
    Wait For Line On Uart   Task1 end: 150