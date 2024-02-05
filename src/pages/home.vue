<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { computed, ref, watch } from "vue";
import { usePickerHistory } from "../state/usePickHistory";
import { NSelect } from "naive-ui";
import color from "color";
import { useClipboard } from "@vueuse/core";

type IColor = [number, number, number, number];


const { colorHistory: historyColors, addColor } = usePickerHistory();

listen<IColor>("select_color", (event) => {
  handleSelectColor(event.payload);
});

const currentColor = ref<IColor>(historyColors.value?.[0] ?? [0, 0, 0, 0]);


function handleSelectColor(color: any) {
  currentColor.value = color;

  addColor(color);
}

function colorToCss(color?: IColor) {
  return color ? `rgba(${color[0]},${color[1]},${color[2]},1)` : "transparent";
}

function startPick() {
  invoke("color_pick");
}

type ColorType = "hex" | "rgb" | "hsl";

const currentColorType = ref<ColorType>("hex");

const showColorText = computed(() => {
  switch (currentColorType.value) {
    case "hex":
      return color(colorToCss(currentColor.value)).hex();
    case "rgb":
      return color(colorToCss(currentColor.value)).rgb().string();
    case "hsl":
      return color(colorToCss(currentColor.value)).hsl().string();
  }
});

const { copy, isSupported } = useClipboard({ source: showColorText });

watch(showColorText, () => {
  if (isSupported.value) {
    copy();
  }
});
</script>

<template>
  <div class="w-full">
    <div class="w-full py-3 flex items-center px-3">
      <div
        @click="startPick"
        class="text-red text-4xl i-tabler-color-picker cursor-pointer"
      ></div>

      <div
        class="w-[180px] h-32px border-2 border-gray-600 border-solid rounded-2 px-2 ml-3"
      >
        {{ showColorText }}
      </div>

      <n-select
        class="w-[100px] ml-2"
        v-model:value="currentColorType"
        :options="[
          { label: 'HEX', value: 'hex' },
          { label: 'RGB', value: 'rgb' },
          { label: 'HSL', value: 'hsl' },
        ]"
      />
      <!-- <div
        @click="startPick"
        class="text-gray text-4xl i-tabler-copy cursor-pointer ml-3"
      ></div> -->
    </div>
    <div class="w-full flex px-3 items-center">
      <div
        class="w-[50px] h-[50px] rounded-1/2 overflow-hidden mr-3 flex-shrink-0"
        :style="{ backgroundColor: colorToCss(currentColor) }"
      ></div>

      <div class="flex-grow flex overflow-hidden flex-nowrap">
        <div
          v-for="item in historyColors"
          class="w-[30px] h-[30px] overflow-hidden flex-shrink-0 cursor-pointer"
          @click="handleSelectColor(item)"
          :style="{ backgroundColor: colorToCss(item) }"
        ></div>
      </div>
    </div>
  </div>
</template>
