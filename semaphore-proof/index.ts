import { Identity } from "@semaphore-protocol/identity";
import { Group } from "@semaphore-protocol/group";
import { generateProof, verifyProof } from "@semaphore-protocol/proof";

function generateIdentity(seed: string = "privKey") {
  const id = new Identity(seed);
  //   const { privateKey, publicKey, commitment } = id;
  //   console.log(privateKey);
  //   console.log(publicKey);
  //   console.log(commitment);
  return id;
}

function signMessage(message: string, id: Identity) {
  const signature = id.signMessage(message);
  return signature;
}

// ID
const id1 = generateIdentity("id1");

let mess = "Hello World";
let sig = signMessage(mess, id1);
let verif = Identity.verifySignature(mess, sig, id1.publicKey);
console.log("verif", verif);

// GROUP
const group1 = new Group();

group1.addMember(id1.commitment);

const id2 = generateIdentity("id2");
const id3 = generateIdentity("id3");

group1.addMember(id2.commitment);
group1.addMember(id3.commitment);

console.log(group1.members);

// merkle proof
let merkleProof = group1.generateMerkleProof(0);
console.assert(merkleProof.leaf == id1.commitment);
console.assert(merkleProof.siblings[0] == id2.commitment);
console.log(merkleProof);

// zk proof
const scope = group1.root;
const message = 1;

async function getProof() {
  const proof1 = await generateProof(id1, group1, message, scope);
  const proof2 = await generateProof(id1, merkleProof, message, scope);
  console.log(proof1);
  console.log(proof2);

  console.log(await verifyProof(proof1));
  console.log(await verifyProof(proof2));
}

getProof();
