export class TytleHost {
  constructor() {
    this.degree = 0;
    this.setx(400);
    this.sety(500);
    this.pen_down();
    this.show_turtle();
  }

  forward(count) {
    const angle_radian = (this.degree * Math.PI) / 180;

    const dx = Math.sin(angle_radian);
    const dy = Math.cos(angle_radian);

    const new_y = this.y - dy * count;
    const new_x = this.x - dx * count;

    this._drawLine(this.x, this.y, new_x, new_y);

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

  show_turtle() {
    this.visible = true;
  }

  hide_turtle() {
    this.visible = false;
  }

  pen_up() {
    this.pen_state = 'UP';
  }

  pen_down() {
    this.pen_state = 'DOWN';
  }

  pen_erase() {
    this.pen_state = 'ERASE';
  }

  clean() {
  }

  clearScreen() {
    var canvas = this._getCanvas();
    var ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }

  _drawLine(x0, y0, x1, y1) {
    var canvas = this._getCanvas();
    var ctx = canvas.getContext("2d");
    ctx.fillStyle = "#FFFFFF";

    switch (this.pen_state) {
      case 'DOWN':
        console.log('PEN IS DOWN');
        ctx.moveTo(x0, y0);
        ctx.lineTo(x1, y1);
        ctx.stroke();
        break;
      case 'UP':
        console.log('PEN IS UP');
        ctx.moveTo(x1, y1);
        break;
      case 'ERASE':
        console.log('PEN ERASE');
        ctx.fillStyle = "#000000";
        break;
    };

    console.log(`moved (${x0}, ${y0}) -> (${x1}, ${y1})`);
  }

  _getCanvas() {
    return document.getElementById("tytle-canvas")
  }
}
