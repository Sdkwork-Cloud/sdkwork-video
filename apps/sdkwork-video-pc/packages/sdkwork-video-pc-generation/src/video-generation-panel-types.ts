import type { SdkworkGenerationSerializedAssetConfig } from './generation-asset-config';
import type { VideoGenerationModelGroup, VideoReferenceAssetInput, VideoReferenceMode } from './video-reference-capability';

export interface VideoGenerationSubmitInput {
  prompt: string;
  selectedModality: 'video';
  targetType?: 'video';
  selectedModel?: string;
  generationConfig?: SdkworkGenerationSerializedAssetConfig;
  referenceAssets?: VideoReferenceAssetInput[];
  referenceMode?: VideoReferenceMode;
}

export interface VideoGenerationPanelProps {
  placeholderKey: string;
  modelGroups: VideoGenerationModelGroup[];
  selectedModelId: string;
  onSubmitGeneration: (input: VideoGenerationSubmitInput) => Promise<void>;
  submitting: boolean;
  submitError: string | null;
}
