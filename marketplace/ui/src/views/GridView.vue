<template>
  <v-container fluid>
    <v-row justify="center" align="center">
      <v-col sm="9">
        <v-text-field
          v-model="$store.state.selectedPixelID"
          type="number"
          :color="$store.state.primaryColor"
          label="Pixel Number"
          hint="0"
          required
        ></v-text-field>
        <v-row align="center" justify="center" no-gutters>
          <v-col cols="8" sm="6">
            <v-btn max-width="100" @click="colorPixel">Color</v-btn>
          </v-col>
          <v-col cols="8" sm="6">
            <v-btn @click="togglePixels">Show Pixel Numbers</v-btn>
          </v-col>
        </v-row>
      </v-col>
    </v-row>
    <v-row justify="center" align="center">
      <canvas
        id="myCanvas"
        class="gridCanvas"
        :width="width"
        :height="height"
      ></canvas
    ></v-row>
    <pixel-modal />
  </v-container>
</template>

<script>
import pixelModal from "../modals/PixelModal.vue";
import web3Utils from "web3-utils";
import bigNumber from "bignumber.js";
export default {
  name: "xon-GridCanvas",
  components: {
    "pixel-modal": pixelModal,
  },
  data() {
    return {
      width: window.innerWidth,
      height: window.innerHeight - 10,
      validPositions: [],
      paintablePoints: [],
      pixelID: 0,
      indexY: 0,
      board: {},
      gridSize: 13000,
      grid: [],
      pixelsToggled: false,
      pixelData: [],
    };
  },

  methods: {
    togglePixels: async function() {
      this.$store.state.isLoading = true;
      /* */ await this.init2();
      this.pixelsToggled = !this.pixelsToggled;
      this.$store.state.isLoading = false;
    },
    init2: async function() {
      let _this = this;
      this.$store.state.isLoading = true;
      var Grid = require("../js/grid/grid.js").Grid;
      var canvas = document.getElementById("myCanvas");
     // window.addEventListener("resize", this.resizeCanvas, false);
     // window.addEventListener("orientationchange", this.resizeCanvas, false);
      this.resizeCanvas();
      const grid = new Grid(
        this.gridSize,
        30,
        canvas,
        this.$store.state.selectedQaudrantStart,
        this.pixelsToggled
      );
      /*  grid.set(
        0,
        0,
        {
          owner: "0x2819c144D5946404C0516B6f817a960dB37D4929",
          currentPrice: 0,
          id: 0,
          fillColor: this.$store.state.pickerColor,
          strokeColor: this.$store.state.pickerColor,
          row: 0,
          col: 0,
        },
        true
      );
       grid.onCellClick = (e, m) => {
        console.log("clickedOn: ", e.row, e.col);

      };*/

      this.grid = grid.getGrid();
      this.board = grid;
      this.$store.state.board = this.board;
      this.$store.state.grid = this.grid;
      var minCost = await _this.$store.state.skyPainter.minPaintCost();
      minCost = new bigNumber(
        _this.$store.state.etherConverter(minCost, "wei", "eth")
      ).toFixed(7);
      var colorPixels = await _this.$store.state.skyPainter.methods
        .getPixelIndexes()
        .call({ from: _this.$store.state.userAddress, gas: 6000000 });
      //console.log("coloredPixels: ", colorPixels);
      for (var cell in this.$store.state.grid) {
        //  console.log("celllll: ", this.$store.state.grid[cell]);
        var innerCells = [];
        for (var innerGridCell in this.$store.state.grid[cell]) {
          var gridCell = this.$store.state.grid[cell][innerGridCell];
          if (colorPixels.includes(gridCell.id.toString())) {
            var pixelDetails = await _this.$store.state.skyPainter.methods
              .getPixelDetails(gridCell.id)
              .call({
                from: _this.$store.state.userAddress,
                gas: 6000000,
              });
            const fillColor = pixelDetails[5];
            const owner = pixelDetails[2];
            const currentPrice = new bigNumber(
              _this.$store.state.etherConverter(pixelDetails[3], "wei", "eth")
            ).toFixed(7);
            const price = new bigNumber(
              _this.$store.state.etherConverter(pixelDetails[4], "wei", "eth")
            ).toFixed(7);
            gridCell.owner = owner;
            gridCell.currentPrice = currentPrice;
            gridCell.fillColor = web3Utils
              .toAscii(fillColor)
              .split("\u0000")[0];
            gridCell.strokeColor = gridCell.fillColor;
            gridCell.price = price;
            grid.set(gridCell.row, gridCell.col, gridCell, true);
            innerCells.push(gridCell);
          } else {
            gridCell.fillColor = "transparent";
            gridCell.strokeColor = "transparent";
            gridCell.price = minCost;
            gridCell.currentPrice = minCost;
            grid.set(gridCell.row, gridCell.col, gridCell, true);
            innerCells.push(gridCell);
          }
        }
        this.pixelData.push(innerCells);
      }
      // console.log("photoGrid: ", this.pixelData);
      this.$store.state.isLoading = false;

      // console.log("grid: ", this.board);
    },
    colorPixel() {
      if (
        parseInt(this.$store.state.selectedPixelID) >= 0 &&
        parseInt(this.$store.state.selectedPixelID) <= 1296
      ) {
        //this.pixelID = this.pixelID == 0 ? 1 : this.pixelID;
        this.$store.state.isLoading = true;
        this.pixelData.map((cell) => {
          // console.log("cell: ", cell);
          var tempCell = cell.filter(
            (gridCell) =>
              gridCell.id === parseInt(this.$store.state.selectedPixelID)
          );
          if (tempCell.length > 0) {
            tempCell = tempCell[0];
            //  console.log(
            //    "tempCell: ",
            //     tempCell,
            //    " isColor: ",
            //   tempCell.fillColor !== null
            //  );
            this.$store.state.isLoading = false;
            this.$store.state.currentPixel = tempCell;
            this.$store.state.showPixelDialog = true;
            if (tempCell.fillColor !== undefined) {
              /*   this.$store.dispatch(
                "warning",
                "The current is colored in would you like to purchase it?"
              );*/
              this.$store.state.currentPixel = tempCell;
            }
          }
          return cell;
        });
      } else {
        var message = {
          warning: "Please enter a Pixel within the size of the canvas",
          onTap: () => {},
        };
        this.$store.dispatch("warning", message);
      }
    },
    resizeCanvas() {
      var canvas = document.getElementById("myCanvas");
      if (canvas) {
        canvas.width = this.width;
        canvas.height = this.height;
      }
    },
  },
  mounted() {
    this.$store.state.loadingZIndex = 0;
    this.init2();
  },
};
</script>

<style scoped>
.gridCanvas {
  position: relative !important;
  border: lightgrey 1px solid;
  border-radius: 5px;
  box-shadow: 0 2px 3px rgba(0, 0, 0, 0.2);
}
body {
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: whitesmoke;
}

canvas {
  border: 1px solid red;
  background-color: rgb(255, 255, 255);
  position: absolute;
  width: 100%;
  height: 100%;
}
</style>
