export class TytleHost {
  constructor() {
    this.x = 0;
    this.y = 0;
  }

  forward(count) {
    const new_y = this.y + count;
    this.drawUpdate(this.x, this.y, this.x, new_y);
    this.y = new_y;
  }

  backward(count) {
    var new_y = this.y - count;

    if (new_y < 0) {
      new_y = 0;
    }

    this.drawUpdate(this.x, this.y, this.x, new_y);
    this.y = new_y;
  }

  left(count) {
    var new_x = this.x - count;

    if (new_x < 0) {
      new_x = 0;
    }

    this.drawUpdate(this.x, this.y, new_x, this.y);
    this.x = new_x;
  }

  right(count) {
    const new_x = this.x + count;
    this.drawUpdate(this.x, this.y, new_x, this.y);
    this.x = new_x;
  }

  setx(x) {
    this.x = x;
  }

  sety(y) {
    this.y = y;
  }

  drawUpdate(x0, y0, x1, y1) {
    var canvas = document.getElementById("tytle_canvas");
    var ctx = canvas.getContext("2d");
    ctx.fillStyle = "#FFFFFF";

    ctx.moveTo(x0, y0);
    ctx.lineTo(x1, y1);
    ctx.stroke();

    console.log(`moved (${x0}, ${y0}) -> (${x1}, ${y1})`);
  }
}
