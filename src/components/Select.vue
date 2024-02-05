<script lang="ts" setup>
import {
  Listbox,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/vue";
import { useVModel } from "@vueuse/core";
import { ref, watchEffect } from "vue";
const props = withDefaults(
  defineProps<{
    value?: string | number;
    options?: Array<{ label: string; value: string | number }>;
  }>(),
  {
    options: () => [],
  }
);

const emits = defineEmits(["update:value"]);

const value = useVModel(props, "value", emits, {
  passive: true,
});

const selected = ref<any>();



watchEffect(() => {
  selected.value = props.options.find((o) => o.value === value.value);
});
</script>

<template>
  <Listbox v-model="selected">
    <ListboxButton>{{ selected?.label }}</ListboxButton>
    <ListboxOptions>
      <ListboxOption v-for="op in props.options" :key="op.value" :value="op">
        {{ op.label }}
      </ListboxOption>
    </ListboxOptions>
  </Listbox>
</template>
