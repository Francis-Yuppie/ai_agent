<script setup>
import { useLayout } from "@/layout/composables/layout";
import { onMounted, ref, watch } from "vue";

const { getPrimary, getSurface, isDarkTheme } = useLayout();

const radarData = ref(null);

const radarOptions = ref(null);

onMounted(() => {
  setColorOptions();
});

function setColorOptions() {
  const documentStyle = getComputedStyle(document.documentElement);
  const textColor = documentStyle.getPropertyValue("--text-color");
  const textColorSecondary = documentStyle.getPropertyValue(
    "--text-color-secondary"
  );
  const surfaceBorder = documentStyle.getPropertyValue("--surface-border");

  radarData.value = {
    labels: [
      "Reentrancy Attacks",
      "Hash Collisions ",
    //   "Privilege Escalation",
      "Loop Exploits",
      "Best Practices",
    //   "Unprotected Functions",
      "Logic Flaws",
    ],
    datasets: [
      {
        label: "Weakness magnitude",
        borderColor: documentStyle.getPropertyValue("--p-purple-400"),
        pointBackgroundColor: documentStyle.getPropertyValue("--p-purple-400"),
        pointBorderColor: documentStyle.getPropertyValue("--p-purple-400"),
        pointHoverBackgroundColor: textColor,
        pointHoverBorderColor: documentStyle.getPropertyValue("--p-purple-400"),
        data: [80, 48, 60, 80, 96, 60, 20],
      },
    ],
  };

  radarOptions.value = {
    plugins: {
      legend: {
        labels: {
          fontColor: textColor,
        },
      },
    },
    scales: {
      r: {
        grid: {
          color: textColorSecondary,
        },
      },
    },
  };
}

watch(
  [getPrimary, getSurface, isDarkTheme],
  () => {
    setColorOptions();
  },
  { immediate: true }
);
</script>

<template>
  <div class="card flex flex-col items-center ">
    <div class="font-semibold text-xl">Painpoints</div>
    <Chart type="radar" :data="radarData" :options="radarOptions"></Chart>
  </div>
</template>




