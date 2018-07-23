use failure::Error;

#[derive(Debug, Fail)]
enum TwsError {
    #[fail(display = "invalid toolchain name: {}", desc)]
 NO_VALID_ID  = { id: -1, desc: "NO_VALID_ID" },
 ALREADY_CONNECTED = 
    { id: 501, desc: 
     desc: "Already connected."},

 CONNECT_FAIL = 
    { id: 502, desc: 
     desc: "Couldn't connect to TWS. Confirm that \"Enable ActiveX and Socket Clients\" "
    		+ "is enabled and connection port is the same as \"Socket Port\" on the TWS \"Edit->Global Configuration...->API->Settings\" menu. "
    		+ "Live Trading ports: TWS: 7496; IB Gateway: 4001. Simulated Trading ports for new installations of version 954.1 or newer: "
    		+ "TWS: 7497; IB Gateway: 4002"},
 UPDATE_TWS = { id: 503, desc: "The TWS is out of date and must be upgraded."},
 NOT_CONNECTED = { id: 504, desc: "Not connected"},
 UNKNOWN_ID = { id: 505, desc: "Fatal Error: Unknown message id."},
 UNSUPPORTED_VERSION = { id: 506, desc: "Unsupported Version"},
 BAD_LENGTH = { id: 507, desc: "Bad Message Length"},
 BAD_MESSAGE = { id: 508, desc: "Bad Message"},
 FAIL_SEND = { id: 509, desc: "Failed to send message - "},
 FAIL_SEND_REQMKT = { id: 510, desc: "Request Market Data Sending Error - "},
 FAIL_SEND_CANMKT = { id: 511, desc: "Cancel Market Data Sending Error - "},
 FAIL_SEND_ORDER = { id: 512, desc: "Order Sending Error - "},
 FAIL_SEND_ACCT = { id: 513, desc: "Account Update Request Sending Error -"},
 FAIL_SEND_EXEC = { id: 514, desc: "Request For Executions Sending Error -"},
 FAIL_SEND_CORDER = { id: 515, desc: "Cancel Order Sending Error -"},
 FAIL_SEND_OORDER = { id: 516, desc: "Request Open Order Sending Error -"},
 UNKNOWN_CONTRACT = { id: 517, desc: "Unknown contract. Verify the contract details supplied."},
 FAIL_SEND_REQCONTRACT = { id: 518, desc: "Request Contract Data Sending Error - "},
 FAIL_SEND_REQMKTDEPTH = { id: 519, desc: "Request Market Depth Sending Error - "},
 FAIL_SEND_CANMKTDEPTH = { id: 520, desc: "Cancel Market Depth Sending Error - "},
 FAIL_SEND_SERVER_LOG_LEVEL = { id: 521, desc: "Set Server Log Level Sending Error - "},
 FAIL_SEND_FA_REQUEST = { id: 522, desc: "FA Information Request Sending Error - "},
 FAIL_SEND_FA_REPLACE = { id: 523, desc: "FA Information Replace Sending Error - "},
 FAIL_SEND_REQSCANNER = { id: 524, desc: "Request Scanner Subscription Sending Error - "},
 FAIL_SEND_CANSCANNER = { id: 525, desc: "Cancel Scanner Subscription Sending Error - "},
 FAIL_SEND_REQSCANNERPARAMETERS = { id: 526, desc: "Request Scanner Parameter Sending Error - "},
 FAIL_SEND_REQHISTDATA = { id: 527, desc: "Request Historical Data Sending Error - "},
 FAIL_SEND_CANHISTDATA = { id: 528, desc: "Request Historical Data Sending Error - "},
 FAIL_SEND_REQRTBARS = { id: 529, desc: "Request Real-time Bar Data Sending Error - "},
 FAIL_SEND_CANRTBARS = { id: 530, desc: "Cancel Real-time Bar Data Sending Error - "},
 FAIL_SEND_REQCURRTIME = { id: 531, desc: "Request Current Time Sending Error - "},
 FAIL_SEND_REQFUNDDATA = { id: 532, desc: "Request Fundamental Data Sending Error - "},
 FAIL_SEND_CANFUNDDATA = { id: 533, desc: "Cancel Fundamental Data Sending Error - "},
 FAIL_SEND_REQCALCIMPLIEDVOLAT = { id: 534, desc: "Request Calculate Implied Volatility Sending Error - "},
 FAIL_SEND_REQCALCOPTIONPRICE = { id: 535, desc: "Request Calculate Option Price Sending Error - "},
 FAIL_SEND_CANCALCIMPLIEDVOLAT = { id: 536, desc: "Cancel Calculate Implied Volatility Sending Error - "},
 FAIL_SEND_CANCALCOPTIONPRICE = { id: 537, desc: "Cancel Calculate Option Price Sending Error - "},
 FAIL_SEND_REQGLOBALCANCEL = { id: 538, desc: "Request Global Cancel Sending Error - "},
 FAIL_SEND_REQMARKETDATATYPE = { id: 539, desc: "Request Market Data Type Sending Error - "},
 FAIL_SEND_REQPOSITIONS = { id: 540, desc: "Request Positions Sending Error - "},
 FAIL_SEND_CANPOSITIONS = { id: 541, desc: "Cancel Positions Sending Error - "},
 FAIL_SEND_REQACCOUNTDATA = { id: 542, desc: "Request Account Data Sending Error - "},
 FAIL_SEND_CANACCOUNTDATA = { id: 543, desc: "Cancel Account Data Sending Error - "},
 FAIL_SEND_VERIFYREQUEST = { id: 544, desc: "Verify Request Sending Error - "},
 FAIL_SEND_VERIFYMESSAGE = { id: 545, desc: "Verify Message Sending Error - "},
 FAIL_SEND_QUERYDISPLAYGROUPS = { id: 546, desc: "Query Display Groups Sending Error - "},
 FAIL_SEND_SUBSCRIBETOGROUPEVENTS = { id: 547, desc: "Subscribe To Group Events Sending Error - "},
 FAIL_SEND_UPDATEDISPLAYGROUP = { id: 548, desc: "Update Display Group Sending Error - "},
 FAIL_SEND_UNSUBSCRIBEFROMGROUPEVENTS = { id: 549, desc: "Unsubscribe From Group Events Sending Error - "},
 FAIL_SEND_STARTAPI = { id: 550, desc: "Start API Sending Error - "},
 FAIL_SEND_VERIFYANDAUTHREQUEST = { id: 551, desc: "Verify And Auth Request Sending Error - "},
 FAIL_SEND_VERIFYANDAUTHMESSAGE = { id: 552, desc: "Verify And Auth Message Sending Error - "},
 FAIL_SEND_REQPOSITIONSMULTI = { id: 553, desc: "Request Positions Multi Sending Error - "},
 FAIL_SEND_CANPOSITIONSMULTI = { id: 554, desc: "Cancel Positions Multi Sending Error - "},
 FAIL_SEND_REQACCOUNTUPDATESMULTI = { id: 555, desc: "Request Account Updates Multi Sending Error - "},
 FAIL_SEND_CANACCOUNTUPDATESMULTI = { id: 556, desc: "Cancel Account Updates Multi Sending Error - "},
 FAIL_SEND_REQSECDEFOPTPARAMS = { id: 557, desc: "Request Security Definition Option Params Sending Error - "},
 FAIL_SEND_REQSOFTDOLLARTIERS = { id: 558, desc: "Request Soft Dollar Tiers Sending Error - "},
}
