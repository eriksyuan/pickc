<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, ref } from "vue";
import color from "color";

const canvas = ref<HTMLCanvasElement>();

type IColor = [number, number, number, number];

const lastColors = ref<IColor[]>([]);

listen<{ colors: IColor[] }>("p_color", (event) => {
  draw(event.payload.colors);
  lastColors.value = event.payload.colors;
});

function draw(colors: IColor[]) {
  if (!canvas.value) return;

  const ctx = canvas.value?.getContext("2d");
  if (!ctx) return;

  const SPACE = 4;

  const width = SPACE * 2 + 1;
  ctx.clearRect(0, 0, 100, 100);
  ctx.fillStyle = `#666`;
  ctx.fillRect(0, 0, 100, 100);

  colors.forEach((c, i) => {
    const x = i % width;
    const y = Math.floor(i / width);
    if (i === Math.floor(colors.length / 2)) {
      ctx.fillStyle = `rgba(0,0,0,1)`;
      ctx.fillRect(x * 11, y * 12 - 4, 12, 12);
    }
    _drawRect(ctx, x * 11 + 1, y * 11 + 1, c);
  });
}

function _drawRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  color: IColor
) {
  ctx.fillStyle = `rgba(${color[0]},${color[1]},${color[2]},${color[3]})`;
  ctx.fillRect(x, y, 10, 10);
}

const colorText = computed(()=>{

  const colroRgb:IColor =  lastColors?.value[41]

  if(!colroRgb) return ''

  return color.rgb(colroRgb).hex()
})

onMounted(() => {});
</script>

<template>
  <div class="container flex flex-col justify-center">
    <div class="h-[36px] w-full"></div>
    <div class="w-[100px] h-[100px] rounded-1/2 overflow-hidden">
      <canvas ref="canvas" width="100" height="100"></canvas>
    </div>
    <div class="h-4 w-full"></div>
    <div class="w-[100px] leading-6 text-center h-6 overflow-hidden text-white rounded-sm bg-gray-900">{{ colorText }}</div>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100%;
}
</style>
