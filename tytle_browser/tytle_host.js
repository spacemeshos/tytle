export class TytleHost {
  constructor() {
    this.x = 0;
    this.y = 0;
    this.angle = 0;

    var canvas = document.getElementById("tytle-canvas");
    var ctx = canvas.getContext("2d");
    ctx.translate(400, 0);
  }

  forward(count) {
    const new_y = this.y + count;
    this.drawLine(this.angle, this.x, this.y, this.x, new_y);
    this.y = new_y;
  }

  backward(count) {
    var new_y = this.y - count;

    if (new_y < 0) {
      new_y = 0;
    }

    this.drawLine(this.angle, this.x, this.y, this.x, new_y);
    this.y = new_y;
  }

  right(angle) {
    this.angle = angle;
  }

  left(angle) {
    this.angle = 360 - angle;
  }

  setx(x) {
    this.x = x;
  }

  sety(y) {
    this.y = y;
  }

  drawLine(angle, x0, y0, x1, y1) {
    var canvas = document.getElementById("tytle-canvas");
    var ctx = canvas.getContext("2d");
    ctx.fillStyle = "#FFFFFF";

    ctx.moveTo(x0, y0);
    ctx.lineTo(x1, y1);
    ctx.stroke();

    console.log(`moved (${x0}, ${y0}) -> (${x1}, ${y1})`);
  }
}
