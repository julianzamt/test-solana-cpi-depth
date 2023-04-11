import { Buffer } from "buffer";
import * as utils from "./utils";

export class Adder {
  constructor(public number: number) {}

  static decode(buffer: Buffer): Adder {
    let newBuffer = utils.copyBuffer(buffer);

    let number;

    [number, newBuffer] = utils.unpackUInt8(newBuffer);

    return new Adder(number);
  }
}
