import { Check, Coins, RectangleHorizontal, RectangleVertical, Square } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import {
  DEFAULT_SDKWORK_GENERATION_VIDEO_MODE_CONFIG,
  type SdkworkGenerationVideoModeConfig,
} from '../generation-asset-config';
import {
  SdkworkGenerationModePopupBase as GenerationModePopupBase,
  type SdkworkGenerationModeSection as ConfigSection,
  formatGenerationCreditPoints,
} from '@sdkwork/generations-pc-studio/react';
export type VideoGenerationConfig = SdkworkGenerationVideoModeConfig;

const VIDEO_SECTION_DEFINITIONS = [
  {
    id: 'resolution',
    labelKey: 'playground.videoSettings.resolution',
    type: 'select' as const,
    valueKey: 'resolution',
    options: [
      { value: '720p', label: '720p' },
      { value: '1080p', label: '1080p', isVip: true },
      { value: '4k', label: '4K', isVip: true },
    ],
  },
  {
    id: 'duration',
    labelKey: 'playground.videoSettings.duration',
    type: 'slider' as const,
    valueKey: 'duration',
    min: 3,
    max: 15,
    step: 1,
    unit: 's',
  },
  {
    id: 'aspectRatio',
    labelKey: 'playground.videoSettings.aspectRatio',
    type: 'select' as const,
    valueKey: 'aspectRatio',
    options: [
      { value: '16:9', label: '16:9', icon: <RectangleHorizontal className="h-4 w-6" /> },
      { value: '1:1', label: '1:1', icon: <Square className="h-4 w-4" /> },
      { value: '9:16', label: '9:16', icon: <RectangleVertical className="h-6 w-4" /> },
    ],
  },
  {
    id: 'count',
    labelKey: 'playground.videoSettings.count',
    type: 'select' as const,
    valueKey: 'count',
    options: [
      { value: 1, label: '1' },
      { value: 2, label: '2', isVip: true },
      { value: 3, label: '3', isVip: true },
      { value: 4, label: '4', isVip: true },
    ],
  },
] as const;

interface VideoGenerationModePopupProps {
  config: VideoGenerationConfig;
  onChangeConfig: (config: VideoGenerationConfig) => void;
  onGenerate: () => void;
  isGenerating?: boolean;
  canGenerate?: boolean;
  showCost?: number;
}

export function VideoGenerationModePopup({
  canGenerate = true,
  config,
  isGenerating = false,
  onChangeConfig,
  onGenerate,
  showCost,
}: VideoGenerationModePopupProps) {
  const { i18n, t } = useTranslation();  const sections = VIDEO_SECTION_DEFINITIONS.map((section) => ({
    ...section,
    label: t(section.labelKey),
  })) satisfies ConfigSection<VideoGenerationConfig>[];
  const getSummary = (current: VideoGenerationConfig) =>
    `${current.resolution} / ${current.duration}s / ${current.aspectRatio} / ${current.count}`;

  return (
    <GenerationModePopupBase
      canGenerate={canGenerate}
      config={config}
      generateLabel={t('playground.generate')}
      generatingLabel={t('playground.videoSettings.generating')}
      getSummary={getSummary}
      isGenerating={isGenerating}
      onChangeConfig={onChangeConfig}
      onGenerate={onGenerate}
      sections={sections}
      title={t('playground.videoSettings.title')}
      barClassName="sdkwork-studio-bottom-bar sdkwork-video-generation-bottom-bar"
      popupClassName="sdkwork-studio-settings-popup sdkwork-video-generation-settings-popup"
      renderExtraControls={() => (
        <>
          <button
            type="button"
            onClick={() => onChangeConfig({ ...config, syncAudioVideo: !config.syncAudioVideo })}
            data-active={config.syncAudioVideo ? 'true' : 'false'}
            className="sdkwork-studio-sync-chip"
          >
            <Check className={`h-3.5 w-3.5 transition-opacity ${config.syncAudioVideo ? 'opacity-100' : 'opacity-0'}`} />
            {t('playground.videoSettings.syncAudioVideo')}
          </button>
          {showCost !== undefined ? (
            <div className="sdkwork-studio-cost">
              <Coins className="h-3.5 w-3.5" />
              <span className="font-bold">{formatGenerationCreditPoints(showCost, i18n.language)}</span>
            </div>
          ) : null}
        </>
      )}
    />  );
}

export const DEFAULT_VIDEO_GENERATION_CONFIG: VideoGenerationConfig = {
  ...DEFAULT_SDKWORK_GENERATION_VIDEO_MODE_CONFIG,
};
