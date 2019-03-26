export class TytleHost {
  constructor() {
    this.degree = 0;
    this.x      = 400;
    this.y      = 500;
  }

  forward(count) {
    const angle_radian = (this.degree * Math.PI) / 180;

    const dx = Math.sin(angle_radian);
    const dy = Math.cos(angle_radian);

    const new_y = this.y - dy * count;
    const new_x = this.x - dx * count;

    this.drawLine(this.x, this.y, new_x, new_y);

    this.x = new_x;
    this.y = new_y;
  }
  backward(count) {
    this.forward((-1) * count);
  }

  left(degree) {
    this.degree += degree;
  }

  right(degree) {
    // counter-clockwise rotation
    this.left((-1) * degree)
  }

  setx(x) {
    this.x = x;
  }

  sety(y) {
    this.y = y;
  }

  drawLine(x0, y0, x1, y1) {
    var canvas = document.getElementById("tytle-canvas");
    var ctx = canvas.getContext("2d");
    ctx.fillStyle = "#FFFFFF";

    ctx.beginPath();
    ctx.moveTo(x0, y0);
    ctx.lineTo(x1, y1);
    ctx.stroke();

    console.log(`moved (${x0}, ${y0}) -> (${x1}, ${y1})`);
  }
}
