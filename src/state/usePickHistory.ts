import { createGlobalState, useStorage } from "@vueuse/core";

const useColorHistoryState = createGlobalState(() =>
  useStorage<any>("picker-history", [])
);

export const usePickerHistory = () => {
  const colorHistory = useColorHistoryState();
  const addColor = (color: any) => {
    colorHistory.value = [color, ...colorHistory.value.slice(0, 9)];
  };
  return {
    colorHistory,
    addColor,
  };
};
