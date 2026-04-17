import { BinaryWriter } from "google-protobuf";

export abstract class Message {
    abstract serializeInternal(bw: BinaryWriter): void;
    abstract mergeFrom(bytes: Uint8Array): this;

    toBinary(): Uint8Array {
        const bw = new BinaryWriter();
        this.serializeInternal(bw);
        return bw.getResultBuffer();
    }
}
