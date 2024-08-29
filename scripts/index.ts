import { UserAddress } from "./constant";
import * as dpit from "./dpit";

const callTheFunction = async () => {
  console.log("Triggering functions , please wait !");
  // ==============================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

  // await dpit.init();
  // await dpit.initEscrows1();
  // await dpit.initEscrows2();
  // await dpit.getTokenSupplies();
  await dpit.getTokenBalances(UserAddress);
  // await dpit.getProgramData();
  // await dpit.mint(UserAddress);
  // await dpit.burn(UserAddress);
  // await dpit.stake(UserAddress);
  // await dpit.getStakedData(UserAddress);
  // await dpit.unstake(UserAddress);

  console.log("\nFunctions Triggered, success !");
  console.log("sent =>>>>>>>>");
  // ==============================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

  // ==============================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
};

callTheFunction();

// npm start run
