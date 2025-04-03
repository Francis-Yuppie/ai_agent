<template>
  <div class="flex  items-center justify-center bg-gray-50 p-4">
    <div
      class="flex h-[80vh] w-full flex-col rounded-lg bg-white shadow-lg"
    >
      <div
        ref="chatBoxRef"
        class="flex-1 overflow-y-auto rounded-t-lg bg-gray-100 p-4"
      >
        <div
          v-for="(message, index) in chat"
          :key="index"
          :class="[
            'flex',
            message.role.user !== undefined ? 'justify-end' : 'justify-start',
            'mb-4',
          ]"
        >
          <div
            v-if="message.role.user === undefined"
            class="mr-2 h-10 w-10 rounded-full"
            :style="{
              backgroundImage: `url(${botImg})`,
              backgroundSize: 'cover',
            }"
          ></div>
          <div
            :class="[
              'max-w-[70%] rounded-lg p-3',
              message.role.user !== undefined
                ? 'bg-blue-500 text-white'
                : 'bg-white shadow',
            ]"
          >
            <div
              :class="[
                'mb-1 flex items-center justify-between text-sm',
                message.role.user !== undefined
                  ? 'text-white'
                  : 'text-gray-500',
              ]"
            >
              <div>
                {{ message.role.user !== undefined ? "User" : "System" }}
              </div>
              <div class="mx-2">{{ formatDate(new Date()) }}</div>
            </div>
            <div>{{ message.content }}</div>
          </div>
          <div
            v-if="message.role.user !== undefined"
            class="ml-2 h-10 w-10 rounded-full"
            :style="{
              backgroundImage: `url(${userImg})`,
              backgroundSize: 'cover',
            }"
          ></div>
        </div>
      </div>
      <form
        @submit.prevent="handleSubmit"
        class="flex rounded-b-lg border-t bg-white p-4"
      >
        <input
          v-model="inputValue"
          type="text"
          class="flex-1 rounded-l border p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Ask anything ..."
          :disabled="isLoading"
        />
        <button
          type="submit"
          class="rounded-r bg-blue-500 p-2 text-white hover:bg-blue-600 disabled:bg-blue-300"
          :disabled="isLoading"
        >
          Send
        </button>
      </form>
    </div>
  </div>
</template>

<script>
import { ref, nextTick, watch } from "vue";
import { wasmalyzer_backend } from "/var/www/html/projects/blockchain/wasmalyzer/src/declarations/wasmalyzer_backend/index";
import botImg from "/bot.svg";
import userImg from "/user.svg";
//   import '@/index.css';

export default {
  setup() {
    const chat = ref([
      {
        role: { system: null },
        content:
          "I'm a Bankai AI agent living on the Internet Computer. Ask me anything.",
      },
    ]);
    const inputValue = ref("");
    const isLoading = ref(false);
    const chatBoxRef = ref(null);

    const formatDate = (date) => {
      const h = "0" + date.getHours();
      const m = "0" + date.getMinutes();
      return `${h.slice(-2)}:${m.slice(-2)}`;
    };

    const askAgent = async (messages) => {
  try {
    // Log the messages being sent
    console.log("Sending messages to the backend:", messages);

    const response = await wasmalyzer_backend.chat(messages);

    // Log the response from the backend
    console.log("Response from the backend:", response);

    // Remove the "thinking" message
    chat.value.pop();

    // Add the new system response message
    chat.value.push({ role: { system: null }, content: response });
  } catch (e) {
    console.error("Error:", e);
    const eStr = String(e);
    const match = eStr.match(/(SysTransient|CanisterReject), \"([^\"]+)/);
    if (match) {
      alert(match[2]);
    }
    chat.value.pop();
  } finally {
    isLoading.value = false;
  }
};


    const handleSubmit = () => {
      if (!inputValue.value.trim() || isLoading.value) return;

      const userMessage = {
        role: { user: null },
        content: inputValue.value,
      };
      const thinkingMessage = {
        role: { system: null },
        content: "Thinking ...",
      };
      chat.value.push(userMessage, thinkingMessage);
      inputValue.value = "";
      isLoading.value = true;
      const messagesToSend = chat.value.slice(1).concat(userMessage);
      askAgent(messagesToSend);
    };

    watch(chat, () => {
      nextTick(() => {
        if (chatBoxRef.value) {
          chatBoxRef.value.scrollTop = chatBoxRef.value.scrollHeight;
        }
      });
    });

    return {
      chat,
      inputValue,
      isLoading,
      chatBoxRef,
      formatDate,
      handleSubmit,
      botImg,
      userImg,
    };
  },
};
</script>

<style scoped>
/* Add any necessary styling here */
</style>
