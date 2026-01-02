// config.ts
export const config: { subway: Array<StopConfig>; bus: Array<StopConfig> } = {
  subway: [
    {
      stop_ids: ["G05S"],
      walk_time: 10,
    },
    {
      stop_ids: ["F03S"],
      walk_time: 14,
    },
  ],
  bus: [
    {
      stop_ids: ["553126"],
      walk_time: 3,
    },
    {
      stop_ids: ["503953"],
      walk_time: 3,
    },
    {
      stop_ids: ["550036", "553036"],
      walk_time: 3,
    },
  ],
};

interface StopConfig {
  stop_ids: Array<string>;
  walk_time: number;
}
