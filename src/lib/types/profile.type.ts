import type z from "zod/v4";
import type { profileSchema } from "../schemas/profile.schema";

export type Profile = z.infer<typeof profileSchema>;

