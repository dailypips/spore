use futures::Future;


pub trait EWrapper {
    fn tickPrice(tickerId: i32, field: i32, price: f64, canAutoExecute: i32);
    fn tickSize( tickerId: i32, field: i32, size: i32);
    fn tickOptionComputation( tickerId: i32, field: i32, impliedVol: f64,
    		delta: f64, optPrice: f64, pvDividend: f64,
    		 gamma: f64,  vega: f64,  theta:f64,  undPrice:f64);
	fn tickGeneric(tickerId: i32, tickType: i32,  value : f64);
	fn tickString(tickerId: i32, tickType: i32, value: String);
	fn tickEFP(tickerId: i32, tickType: i32,  basisPoints : f64,
			 formattedBasisPoints: &str,  impliedFuture : f64, holdDays: i32,
			 futureLastTradeDate: &str,  dividendImpact :f64,  dividendsToLastTradeDate: f64);
    fn orderStatus( orderId: i32, status: &str,  filled: f64,  remaining: f64,
             avgFillPrice: f64,  permId: i32,  parentId: i32,  lastFillPrice: f64,
             clientId: i32,  whyHeld: &str);
    fn openOrder( orderId: i32, contract: &Contract,  order: &Order, orderState: &OrderState );
    fn openOrderEnd();
    fn updateAccountValue(key: &str, value: &str, currency: &str, accountName: &str);
    fn updatePortfolio(contract: &Contract,  position: f64,  marketPrice: f64,  marketValue: f64,
             averageCost: f64,  unrealizedPNL: f64,  realizedPNL: f64, accountName: &str);
    fn updateAccountTime( timeStamp: &str);
    fn accountDownloadEnd(accountName: &str);
    fn nextValidId( orderId: i32);
    fn contractDetails( reqId: i32,  contractDetails: &ContractDetails);
    fn bondContractDetails(reqId: i32,  contractDetails: &ContractDetails);
    fn contractDetailsEnd(reqId: i32);
    fn execDetails( reqId: i32, contract: &Contract,  execution: &Execution);
    fn execDetailsEnd( reqId: i32);
    fn updateMktDepth( tickerId: i32,  position:i32,  operation: i32,  side: i32,  price: f64, size: i32);
    fn updateMktDepthL2( tickerId: i32,  position: i32,  marketMaker: &str,  operation:i32,
    		 side:i32,  price:f64, size: i32);
    fn updateNewsBulletin(  msgId: i32,  msgType:i32,  message: &str,  origExchange: &str);
    fn managedAccounts(  accountsList: &str); //managedAccounts(  accounts: &Vec<String>);
    fn receiveFA( faDataType:i32,  xml: &str);
    fn historicalData(reqId: i32,  date: &str, open: f64, high:f64, low:f64,
                       close: f64,  volume: i32,  count: i32, wap: f64,  hasGaps: bool);
    fn scannerParameters( xml: &str);
    fn scannerData(reqId: i32, rank: i32,  contractDetails: &ContractDetails,  distance: &str,
    		 benchmark: &str,  projection: &str,  legsStr: &str);
    fn scannerDataEnd(reqId: i32);
    fn realtimeBar(reqId: i32, time: i64, open: f64, high:f64, low:f64, close: f64, volume: f64, wap: f64, count: i32);
    fn currentTime(time: i64);
    fn fundamentalData(reqId: i32,  data: &str);
    fn deltaNeutralValidation(reqId: i32,  underComp: &DeltaNeutralContract);
    fn tickSnapshotEnd(reqId: i32);
    fn marketDataType(reqId: i32,  marketDataType: i32);
    fn commissionReport( commissionReport: &CommissionReport);
    fn position(account: &str, contract: &Contract,  pos: f64,  avgCost:f64);
    fn positionEnd();
    fn accountSummary(reqId: i32, account: &str, tag: &str, value: &str, currency: &str);
    fn accountSummaryEnd(reqId: i32);
    fn verifyMessageAPI(  apiData: &str);
    fn verifyCompleted(  isSuccessful:bool,  errorText: &str);
    fn verifyAndAuthMessageAPI(  apiData: &str,  xyzChallange: &str);
    fn verifyAndAuthCompleted(  isSuccessful: bool,  errorText: &str);
    fn displayGroupList( reqId: i32,  groups: &str);
    fn displayGroupUpdated( reqId: i32,  contractInfo: &str);
    fn error(id: i32,  errorCode: i32,  errorMsg: &str);
    fn connectionClosed();
    fn connectAck();
    fn positionMulti( reqId: i32, account: &str, modelCode: &str, contract: &Contract,  pos,  avgCost);
    fn positionMultiEnd( reqId: i32);
    fn accountUpdateMulti( reqId: i32, account: &str, modelCode: &str,  key: &str, value: &str, currency: &str);
    fn accountUpdateMultiEnd( reqId: i32);
    fn securityDefinitionOptionalParameter(reqId: i32,  exchange: &str,  underlyingConId: i32,  tradingClass: &str,  multiplier: &str, Set<String> expirations, Set<> strikes);
    fn securityDefinitionOptionalParameterEnd(reqId: i32);
	fn softDollarTiers(reqId: i32,  tiers: &Vec<SoftDollarTier>);
}






fn connect(tws_addr: SocketAddr) -> impl Future<Self, std::io::Error> {

}

fn run(socket: TcpStream, from_tws, to_tws) -> ExitReason -> impl Future<Item = Self, Err = failure::Err>
{

    let frame_client = handshake.and_then(|socket| {
        let frame: length_delimited::Framed<_, BytesMut> = length_delimited::Builder::new()
            .big_endian()
            .new_framed(socket);
        let (to_socket, from_socket) = frame.split();
        // For each incoming message...
        let reader = from_socket.for_each(move |msg| {
           
            if let Err(_) = from_tws.send(msg) {
                println!("cannot send to message queue");
            }

            Ok(())
        });

        let writer = to_tws
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_socket, |to_socket, msg| to_socket.send(msg))
            .map(|_| ());

        // Use select to allow either the reading or writing half dropping to drop the other
        // half. The `map` and `map_err` here effectively force this drop.
        reader
            .select(writer)
            .map_err(|e| eprintln!("Read Error:"))
            .map(|_| ())
    });
}