<template>
  <v-row justify="center">
    <v-dialog
      v-model="$store.state.showPixelDialog"
      persistent
      max-width="100%"
    >
      <v-card>
        <v-card-title class="text-h5">
          Paint Pixel
        </v-card-title>
        <v-card-text
          >Paint Pixel <b>{{ $store.state.currentPixel.id }}</b> or Purchase the
          Pixel from the current Owner.</v-card-text
        >
        <v-container fluid>
          <v-form ref="form" v-model="valid" lazy-validation>
            <v-text-field
              label="Current Owner"
              v-model="$store.state.currentPixel.owner"
              readonly
              :color="$store.state.primaryColor"
            ></v-text-field>
            <v-text-field
              label="Current Price (ETH)"
              v-model="$store.state.currentPixel.currentPrice"
              readonly
              :color="$store.state.primaryColor"
            ></v-text-field>
            <v-text-field
              v-model="offerPrice"
              :rules="priceRules"
              label="Price Offering (ETH)"
              required
              :color="$store.state.primaryColor"
            ></v-text-field>
            <v-text-field
              v-model="twitterUserName"
              :rules="usernameRules"
              label="Twitter Username"
              hint="e.g. brianspha_"
              required
              :color="$store.state.primaryColor"
            ></v-text-field>
            <div class="form__field">
              <div class="form__label">
                <strong>Please choose a color:</strong>
              </div>
              <div class="form__input">
                <v-swatches
                  v-model="$store.state.pickerColor"
                  show-fallback
                  fallback-input-type="color"
                ></v-swatches>
              </div>
            </div>
          </v-form>
        </v-container>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn text @click="$store.state.showPixelDialog = false">
            Close
          </v-btn>
          <v-btn
            v-if="valid"
            :color="$store.state.primaryColor"
            text
            @click="purchaseColor"
          >
            Purchase
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-row>
</template>

<script>
import VSwatches from "vue-swatches";

// Import the styles too, typically in App.vue or main.js
import "vue-swatches/dist/vue-swatches.css";
import utils from "web3-utils";
import bigNumber from "bignumber.js";
export default {
  components: { VSwatches },
  data() {
    return {
      twitterUserName: "",
      usernameRules: [
        (v) => !!v || "Username required",
        (v) =>
          (v && v.length >= 4 && v.length <= 15) ||
          "Twitter username must be atleast 4 characters or less than equal to 15 characters",
      ],
      pickerColor: "#A463BF",
      valid: false,
      offerPrice: 0,
      priceRules: [
        (v) => !!v || "Offer Price is required",
        (v) =>
          (v && !isNaN(v) && parseFloat(v) > 0) ||
          "Offer Price must be a valid amount",
      ],
    };
  },
  mounted() {
    console.log("currentPixel: ", this.$store.state.currentPixel);
  },
  methods: {
    hexToRgb(hex) {
      var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
      return result
        ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16),
          }
        : null;
    },
    purchaseColor: async function() {
      let _this = this;
      this.$store.state.isLoading = true;
      if (this.$refs.form.validate()) {
        if (
          this.$store.state.currentPixel.owner === this.$store.state.userAddress
        ) {
          var error = {
            error: "You cannot purchase a pixel you already own",
            onTap: () => {},
          };
          this.$store.dispatch("error", error);
        } else if (
          (parseFloat(this.offerPrice) === parseFloat(0) &&
            parseFloat(this.$store.state.currentPixel.offerPrice) ===
              parseFloat(this.offerPrice)) ||
          parseFloat(this.$store.state.currentPixel.currentPrice) >=
            parseFloat(this.offerPrice)
        ) {
          this.$store.state.isLoading = false;
          var message = {
            warning:
              "Please ensure that the offer Price is greater than 0 or the current Offer Price and your twitter username is provided",
            onTap: () => {},
          };
          this.$store.dispatch("warning", message);
        } else {
          console.log("currentPixel: ", this.$store.state.currentPixel);
          this.$store.state.currentPixel.offerPrice = parseFloat(
            this.offerPrice
          );
          this.$store.state.currentPixel.currentPrice = parseFloat(
            this.offerPrice
          );
          this.$store.state.showPixelDialog = false;
          this.$store.state.currentPixel.isPassable = true;
          (this.$store.state.currentPixel.fillColor = this.$store.state.pickerColor),
            (this.$store.state.currentPixel.strokeColor = this.$store.state.pickerColor);
          var prevOwners = await _this.$store.state.skyPainter.methods
            .getPixelDetails(_this.$store.state.currentPixel.id)
            .call({
              from: _this.$store.state.userAddress,
              gas: 6000000,
            });
          prevOwners = prevOwners[6];
          prevOwners.push(_this.$store.state.userAddress);
          var pixelCost = _this.$store.state.etherConverter(
            _this.offerPrice,
            "eth",
            "wei"
          );
          var nftURI = JSON.stringify({
            owner: _this.$store.state.userAddress,
            previousOwners: prevOwners,
            price: new bigNumber(_this.$store.state.currentPixel.currentPrice)
              .minus(
                new bigNumber(
                  _this.$store.state.currentPixel.offerPrice
                ).multipliedBy(0.1)
              )
              .toFixed(7),
            color: _this.$store.state.currentPixel.fillColor,
          });
          _this.$store.state.skyPainter.methods
            .colorPixel(
              _this.$store.state.currentPixel.id,
              utils.toHex(_this.$store.state.currentPixel.fillColor),
              nftURI
            )
            .send({
              from: _this.$store.state.userAddress,
              gas: 6000000,
              value: pixelCost,
            })
            .then(async (receipt, error) => {
              console.log("receipt from coloring pixel: ", receipt);
              var data = await _this.$store.dispatch("getSkyData");
              _this.$store.state.loadingZIndex = 0;

              if (data.data === null) {
                data.data = {
                  users: [
                    {
                      wallet: _this.$store.state.userAddress,
                      twitter_username: _this.twitterUserName,
                      pixels_owned: 1,
                      total_spent: pixelCost,
                    },
                  ],
                };
              } else {
                data.data = data.data.users.map((user) => {
                  if (user.wallet === _this.$store.state.userAddress) {
                    user.total_spent = new bigNumber(user.total_spent)
                      .plus(pixelCost)
                      .toFixed(7);
                    user.pixels_owned = new bigNumber(user.pixels_owned)
                      .plus(1)
                      .toFixed(0);
                  }
                  return user;
                });
              }
              console.log(data.data)
              var results = await _this.$store.dispatch("saveData", data.data);
              _this.$store.state.board.set(
                _this.$store.state.currentPixel.row,
                _this.$store.state.currentPixel.col,
                _this.$store.state.currentPixel,
                true
              );
              _this.$store.state.isLoading = false;
              _this.$store.dispatch("success", "Succesfully color pixel");
              this.$router.push('Grid')
            })
            .catch((error) => {
              console.log("error coloring pixel: ", error);
              _this.$store.state.isLoading = false;
              _this.$store.state.loadingZIndex = 0;
              // console.log("Something went wrong whilst coloring your pixel");
              message = {
                error:
                  "Something went wrong whilst coloring pixel, Please ensure your not attempting to color a pixel you already own",
                onTap: () => {
                  _this.$store.state.showPixelDialog = true;
                },
              };
              _this.$store.dispatch("error", message);
            });
        }
      } else {
        this.$store.state.isLoading = false;
        message = {
          warning:
            "Please ensure that the offer Price is greater than 0 or the current Offer Price",
          onTap: () => {
            this.$store.state.showPixelDialog = true;
          },
        };
        this.$store.dispatch("warning", message);
      }
    },
  },
};
</script>

<style></style>
