import z from "zod/v4";

export const setupSchema = z.object({
  apiKey: z
    .string()
    .nonempty("API Key is required")
    .refine((v) => v.startsWith("re_"), {
      message: "API Key must start with 're_'",
    })
    .length(36, "API Key must be exactly 36 characters long"),
  password: z
    .string()
    .min(8, "Password must be at least 8 characters")
    .max(128, "Password must be less than 128 characters"),
  confirmPassword: z.string(),
});
