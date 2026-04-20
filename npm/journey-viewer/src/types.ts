// Types generated/derived from schema/manifest.schema.json. Keep in sync with
// phenotype-journey-core's serde types.

export interface Step {
  index: number;
  slug: string;
  intent: string;
  screenshot_path: string;
  description?: string | null;
  judge_score?: number | null;
}

export interface Verification {
  overall_score: number;
  describe_confidence: number;
  judge_confidence: number;
  all_intents_passed: boolean;
  mode: "mock" | "api";
  timestamp: string;
}

export interface Manifest {
  id: string;
  intent: string;
  recording?: string | null;
  recording_gif?: string | null;
  keyframe_count?: number;
  passed?: boolean;
  steps?: Step[];
  verification?: Verification | null;
  // Legacy field names used by hwLedger's original JourneyViewer:
  title?: string;
  pass?: boolean;
  keyframes?: Array<{ path: string; caption: string }>;
  error?: string;
}
