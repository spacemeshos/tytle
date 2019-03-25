export class TytleHost {
  constructor() {
  }

  forward(x) {
    console.log("[TytleHost] FORWARD " + x);
  }

  backward(x) {
    console.log("[TytleHost] BACKWARD " + x);
  }

  left(x) {
    console.log("[TytleHost] LEFT " + x);
  }

  right(x) {
    console.log("[TytleHost] RIGHT " + x);
  }

  setx(x) {
    console.log("[TytleHost] SETX " + x);
  }

  sety(x) {
    console.log("[TytleHost] SETY " + x);
  }
}
