import Vue from "vue";
import Vuex from "vuex";
import createPersistedState from "vuex-persistedstate";
import swal from "sweetalert2";
import { SkynetClient, genKeyPairFromSeed } from "skynet-js";
const { publicKey, privateKey } = genKeyPairFromSeed(
  process.env.VUE_APP_APP_SECRET
);
const client = new SkynetClient("https://siasky.net/");

Vue.use(Vuex);
console.log("appsecret: ", process.env.VUE_APP_APP_SECRET);
/* eslint-disable no-new */
const store = new Vuex.Store({
  state: {
    appSecret: process.env.VUE_APP_APP_SECRET,
    skyClient: client,
    privateKey: privateKey,
    publicKey: publicKey,
    etherConverter: require("ether-converter"),
    utils: require("web3-utils"),
    skyPainter: require("../../contracts/embarkArtifacts/contracts/SkyPainter")
      .default,
    connected: false,
    currentPixelPage: 1,
    currentPixelMaxStake: (Math.random() * 100).toFixed(7),
    currentPixelBalance: (Math.random() * 40).toFixed(7),
    totalStaked: 0,
    isLoading: false,
    showPixelDialog: false,
    primaryColor: "purple darken-2",
    pickerColor: "#A463BF",
    currentPixel: {},
    selectedPixelID: 0,
    board: [],
    grid: {},
    boardOwned: [],
    gridOwned: {},
    loadingZIndex: 0,
    userAddress: "",
    selectedQaudrantStart: 0,
    revision:1
  },
  plugins: [createPersistedState()],
  modules: {},
  actions: {
    getSkyData: async function() {
      var test = await this.state.skyClient.db.getJSON(
        this.state.publicKey,
        this.state.appSecret
      );
      return test;
    },
    saveData: async function(context, data) {
      const results = await client.db.setJSON(
        this.state.privateKey,
        this.state.appSecret,
        data,
        BigInt(this.state.revision)
      );
      console.log("results: ", results);
    },
    success(context, message) {
      swal.fire({
        position: "top-end",
        icon: "success",
        title: "Success",
        showConfirmButton: false,
        timer: 2500,
        text: message,
      });
    },
    warning(context, message) {
      console.log("shwoing success message: ", message);
      swal.fire("Warning", message.warning, "warning").then((result) => {
        /* Read more about isConfirmed, isDenied below */
        if (result.isConfirmed) {
          message.onTap();
        }
      });
    },
    error(context, message) {
      console.log("shwoing error message: ", message.error);
      swal.fire("Error!", message.error, "error").then((result) => {
        /* Read more about isConfirmed, isDenied below */
        if (result.isConfirmed) {
          console.log("leveled");
        }
      });
    },
    successWithFooter(context, message) {
      console.log("shwoing successWithFooter message: ", message);
      swal.fire({
        icon: "success",
        title: "Success",
        text: message.message,
        footer: `<a href= https://testnet.bscscan.com/tx/${message.txHash}> View on Binance Explorer</a>`,
      });
    },
    errorWithFooterMetamask(context, message) {
      console.log("shwoing successWithFooter message: ", message);
      swal.fire({
        icon: "error",
        title: "Error!",
        text: message,
        footer: `<a href= https://metamask.io> Download Metamask</a>`,
      });
    },
  },
});

export default store;
