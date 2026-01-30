import Ajv from "ajv";
import stampSchema from "../../core/specs/aln_did_stamp_schema.json" assert { type: "json" };

const ajv = new Ajv({ allErrors: true, strict: true });
const validateStamp = ajv.compile(stampSchema);

export function validateAlnDidStampJson(stampJson) {
  const valid = validateStamp(stampJson);
  if (!valid) {
    const err = new Error("Stamp JSON validation failed");
    err.details = validateStamp.errors;
    throw err;
  }
  return true;
}
