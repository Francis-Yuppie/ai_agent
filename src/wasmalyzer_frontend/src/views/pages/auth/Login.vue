<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { AuthClient } from '@dfinity/auth-client';

const router = useRouter();
const email = ref('');
const password = ref('');
const checked = ref(false);

const loginWithInternetIdentity = async () => {
  console.log('Starting Internet Identity login'); // Debug log
  const authClient = await AuthClient.create();
  await authClient.login({
    identityProvider: `https://identity.ic0.app/?canisterId=${import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY}`,
    onSuccess: () => {
      console.log('Login successful'); // Debug log
      // Redirect to dashboard
      console.log('Redirecting to /dashboard'); // Debug log
      router.push('/dashboard');
    },
    onError: (error) => {
      console.error('Login failed', error); // Debug log
    },
  });
};
</script>

<template>
    <FloatingConfigurator />
    <div class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 flex items-center justify-center min-h-screen min-w-[100vw] overflow-hidden relative">
        <!-- Animated background particles -->
        <div class="absolute inset-0 overflow-hidden">
            <div class="particles-container">
                <div v-for="i in 40" :key="i" class="particle"></div>
            </div>
        </div>
        
        <!-- Neural network pattern overlay -->
        <div class="absolute inset-0 bg-neural-pattern opacity-10"></div>
        
        <div class="flex flex-col items-center justify-center relative z-10">
            <div class="glow-effect">
                <div class="w-full bg-white py-16 px-8 sm:px-16 backdrop-blur-xl bg-opacity-95" style="border-radius: 24px; box-shadow: 0 8px 32px rgba(31, 38, 135, 0.1);">
                    <div class="text-center mb-8">
                        <!-- AI-themed animated logo -->
                        <div class="mb-8 mx-auto w-24 h-24 relative">
                            <svg viewBox="0 0 54 40" fill="none" xmlns="http://www.w3.org/2000/svg" class="pulse-effect w-full h-full">
                                <path
                                    fill-rule="evenodd"
                                    clip-rule="evenodd"
                                    d="M17.1637 19.2467C17.1566 19.4033 17.1529 19.561 17.1529 19.7194C17.1529 25.3503 21.7203 29.915 27.3546 29.915C32.9887 29.915 37.5561 25.3503 37.5561 19.7194C37.5561 19.5572 37.5524 19.3959 37.5449 19.2355C38.5617 19.0801 39.5759 18.9013 40.5867 18.6994L40.6926 18.6782C40.7191 19.0218 40.7326 19.369 40.7326 19.7194C40.7326 27.1036 34.743 33.0896 27.3546 33.0896C19.966 33.0896 13.9765 27.1036 13.9765 19.7194C13.9765 19.374 13.9896 19.0316 14.0154 18.6927L14.0486 18.6994C15.0837 18.9062 16.1223 19.0886 17.1637 19.2467ZM33.3284 11.4538C31.6493 10.2396 29.5855 9.52381 27.3546 9.52381C25.1195 9.52381 23.0524 10.2421 21.3717 11.4603C20.0078 11.3232 18.6475 11.1387 17.2933 10.907C19.7453 8.11308 23.3438 6.34921 27.3546 6.34921C31.36 6.34921 34.9543 8.10844 37.4061 10.896C36.0521 11.1292 34.692 11.3152 33.3284 11.4538ZM43.826 18.0518C43.881 18.6003 43.9091 19.1566 43.9091 19.7194C43.9091 28.8568 36.4973 36.2642 27.3546 36.2642C18.2117 36.2642 10.8 28.8568 10.8 19.7194C10.8 19.1615 10.8276 18.61 10.8816 18.0663L7.75383 17.4411C7.66775 18.1886 7.62354 18.9488 7.62354 19.7194C7.62354 30.6102 16.4574 39.4388 27.3546 39.4388C38.2517 39.4388 47.0855 30.6102 47.0855 19.7194C47.0855 18.9439 47.0407 18.1789 46.9536 17.4267L43.826 18.0518ZM44.2613 9.54743L40.9084 10.2176C37.9134 5.95821 32.9593 3.1746 27.3546 3.1746C21.7442 3.1746 16.7856 5.96385 13.7915 10.2305L10.4399 9.56057C13.892 3.83178 20.1756 0 27.3546 0C34.5281 0 40.8075 3.82591 44.2613 9.54743Z"
                                    fill="url(#gradient-fill)"
                                />
                                <mask id="mask0_1413_1551" style="mask-type: alpha" maskUnits="userSpaceOnUse" x="0" y="8" width="54" height="11">
                                    <path d="M27 18.3652C10.5114 19.1944 0 8.88892 0 8.88892C0 8.88892 16.5176 14.5866 27 14.5866C37.4824 14.5866 54 8.88892 54 8.88892C54 8.88892 43.4886 17.5361 27 18.3652Z" fill="url(#gradient-fill)" />
                                </mask>
                                <g mask="url(#mask0_1413_1551)">
                                    <path
                                        d="M-4.673e-05 8.88887L3.73084 -1.91434L-8.00806 17.0473L-4.673e-05 8.88887ZM27 18.3652L26.4253 6.95109L27 18.3652ZM54 8.88887L61.2673 17.7127L50.2691 -1.91434L54 8.88887ZM-4.673e-05 8.88887C-8.00806 17.0473 -8.00469 17.0505 -8.00132 17.0538C-8.00018 17.055 -7.99675 17.0583 -7.9944 17.0607C-7.98963 17.0653 -7.98474 17.0701 -7.97966 17.075C-7.96949 17.0849 -7.95863 17.0955 -7.94707 17.1066C-7.92401 17.129 -7.89809 17.1539 -7.86944 17.1812C-7.8122 17.236 -7.74377 17.3005 -7.66436 17.3743C-7.50567 17.5218 -7.30269 17.7063 -7.05645 17.9221C-6.56467 18.3532 -5.89662 18.9125 -5.06089 19.5534C-3.39603 20.83 -1.02575 22.4605 1.98012 24.0457C7.97874 27.2091 16.7723 30.3226 27.5746 29.7793L26.4253 6.95109C20.7391 7.23699 16.0326 5.61231 12.6534 3.83024C10.9703 2.94267 9.68222 2.04866 8.86091 1.41888C8.45356 1.10653 8.17155 0.867278 8.0241 0.738027C7.95072 0.673671 7.91178 0.637576 7.90841 0.634492C7.90682 0.63298 7.91419 0.639805 7.93071 0.65557C7.93897 0.663455 7.94952 0.673589 7.96235 0.686039C7.96883 0.692262 7.97582 0.699075 7.98338 0.706471C7.98719 0.710167 7.99113 0.714014 7.99526 0.718014C7.99729 0.720008 8.00047 0.723119 8.00148 0.724116C8.00466 0.727265 8.00796 0.730446 -4.673e-05 8.88887ZM27.5746 29.7793C37.6904 29.2706 45.9416 26.3684 51.6602 23.6054C54.5296 22.2191 56.8064 20.8465 58.4186 19.7784C59.2265 19.2431 59.873 18.7805 60.3494 18.4257C60.5878 18.2482 60.7841 18.0971 60.9374 17.977C61.014 17.9169 61.0799 17.8645 61.1349 17.8203C61.1624 17.7981 61.1872 17.7781 61.2093 17.7602C61.2203 17.7512 61.2307 17.7427 61.2403 17.7348C61.2452 17.7308 61.2499 17.727 61.2544 17.7233C61.2566 17.7215 61.2598 17.7188 61.261 17.7179C61.2642 17.7153 61.2673 17.7127 54 8.88887C46.7326 0.0650536 46.7357 0.0625219 46.7387 0.0600241C46.7397 0.0592345 46.7427 0.0567658 46.7446 0.0551857C46.7485 0.0520238 46.7521 0.0489887 46.7557 0.0460799C46.7628 0.0402623 46.7694 0.0349487 46.7753 0.0301318C46.7871 0.0204986 46.7966 0.0128495 46.8037 0.00712562C46.818 -0.00431848 46.8228 -0.00808311 46.8184 -0.00463784C46.8096 0.00228345 46.764 0.0378652 46.6828 0.0983779C46.5199 0.219675 46.2165 0.439161 45.7812 0.727519C44.9072 1.30663 43.5257 2.14765 41.7061 3.02677C38.0469 4.79468 32.7981 6.63058 26.4253 6.95109L27.5746 29.7793ZM54 8.88887C50.2691 -1.91433 50.27 -1.91467 50.271 -1.91498C50.2712 -1.91506 50.272 -1.91535 50.2724 -1.9155C50.2733 -1.91581 50.274 -1.91602 50.2743 -1.91616C50.2752 -1.91643 50.275 -1.91636 50.2738 -1.91595C50.2714 -1.91515 50.2652 -1.91302 50.2552 -1.9096C50.2351 -1.90276 50.1999 -1.89078 50.1503 -1.874C50.0509 -1.84043 49.8938 -1.78773 49.6844 -1.71863C49.2652 -1.58031 48.6387 -1.377 47.8481 -1.13035C46.2609 -0.635237 44.0427 0.0249875 41.5325 0.6823C36.215 2.07471 30.6736 3.15796 27 3.15796V26.0151C33.8087 26.0151 41.7672 24.2495 47.3292 22.7931C50.2586 22.026 52.825 21.2618 54.6625 20.6886C55.5842 20.4011 56.33 20.1593 56.8551 19.986C57.1178 19.8993 57.3258 19.8296 57.4735 19.7797C57.5474 19.7548 57.6062 19.7348 57.6493 19.72C57.6709 19.7127 57.6885 19.7066 57.7021 19.7019C57.7089 19.6996 57.7147 19.6976 57.7195 19.696C57.7219 19.6952 57.7241 19.6944 57.726 19.6938C57.7269 19.6934 57.7281 19.693 57.7286 19.6929C57.7298 19.6924 57.7309 19.692 54 8.88887ZM27 3.15796C23.3263 3.15796 17.7849 2.07471 12.4674 0.6823C9.95717 0.0249875 7.73904 -0.635237 6.15184 -1.13035C5.36118 -1.377 4.73467 -1.58031 4.3155 -1.71863C4.10609 -1.78773 3.94899 -1.84043 3.84961 -1.874C3.79994 -1.89078 3.76474 -1.90276 3.74471 -1.9096C3.73469 -1.91302 3.72848 -1.91515 3.72613 -1.91595C3.72496 -1.91636 3.72476 -1.91643 3.72554 -1.91616C3.72593 -1.91602 3.72657 -1.91581 3.72745 -1.9155C3.72789 -1.91535 3.72874 -1.91506 3.72896 -1.91498C3.72987 -1.91467 3.73084 -1.91433 -4.673e-05 8.88887C-3.73093 19.692 -3.72983 19.6924 -3.72868 19.6929C-3.72821 19.693 -3.72698 19.6934 -3.72603 19.6938C-3.72415 19.6944 -3.72201 19.6952 -3.71961 19.696C-3.71482 19.6976 -3.70901 19.6996 -3.7022 19.7019C-3.68858 19.7066 -3.67095 19.7127 -3.6494 19.72C-3.60629 19.7348 -3.54745 19.7548 -3.47359 19.7797C-3.32589 19.8296 -3.11788 19.8993 -2.85516 19.986C-2.33008 20.1593 -1.58425 20.4011 -0.662589 20.6886C1.17485 21.2618 3.74125 22.026 6.67073 22.7931C12.2327 24.2495 20.1913 26.0151 27 26.0151V3.15796Z"
                                        fill="url(#gradient-fill)"
                                    />
                                </g>
                                <!-- Add gradient definitions -->
                                <defs>
                                    <linearGradient id="gradient-fill" x1="0" y1="0" x2="54" y2="40" gradientUnits="userSpaceOnUse">
                                        <stop offset="0%" stop-color="#4F46E5"/>
                                        <stop offset="50%" stop-color="#7C3AED"/>
                                        <stop offset="100%" stop-color="#DB2777"/>
                                    </linearGradient>
                                </defs>
                            </svg>
                            <!-- Circular glow effect -->
                            <div class="absolute inset-0 pulse-ring"></div>
                        </div>
                        <div class="text-gray-900 text-4xl font-bold mb-3">Sakai</div>
                        <span class="text-gray-600 font-medium text-lg">The next generation of AI-powered experiences</span>
                    </div>

                    <!-- Internet Identity Login Button -->
                    <div class="mb-8">
                        <Button 
                            @click="loginWithInternetIdentity" 
                            class="w-full mb-5 bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 border-none text-white py-3 px-6 text-lg font-medium" 
                            severity="primary" 
                            label="Sign in with Internet Identity"
                        >
                            <template #icon>
                                <i class="pi pi-shield mr-2"></i>
                            </template>
                        </Button>
                        
                        <div class="flex items-center justify-between mb-5">
                            <span class="border-b border-gray-500 w-1/5 lg:w-1/4"></span>
                            <span class="text-sm text-center text-blue-200 uppercase tracking-wider">or login with email</span>
                            <span class="border-b border-gray-500 w-1/5 lg:w-1/4"></span>
                        </div>
                    </div>

                    <div class="text-gray-900">
                        <label for="email1" class="block text-gray-700 text-xl font-medium mb-2">Email</label>
                        <InputText id="email1" type="text" placeholder="Your email address" class="w-full md:w-full mb-6 p-3 rounded-lg bg-white border border-gray-300 focus:border-blue-500 placeholder-gray-400" v-model="email" />

                        <label for="password1" class="block text-gray-700 font-medium text-xl mb-2">Password</label>
                        <Password id="password1" v-model="password" placeholder="Your password" :toggleMask="true" class="mb-5 custom-password" :feedback="false"></Password>

                        <div class="flex items-center justify-between mt-4 mb-8 gap-8">
                            <div class="flex items-center">
                                <Checkbox v-model="checked" id="rememberme1" binary class="mr-2"></Checkbox>
                                <label for="rememberme1" class="text-gray-600">Remember me</label>
                            </div>
                            <span class="font-medium no-underline ml-2 text-right cursor-pointer text-blue-600 hover:text-blue-800 transition-colors duration-200">Forgot password?</span>
                        </div>
                        <Button label="Sign In" class="w-full py-3 text-lg font-semibold bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 border-none" as="router-link" to="/">
                            <template #icon>
                                <i class="pi pi-lock-open mr-2"></i>
                            </template>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Make sure both input fields have the same width */
:deep(.p-inputtext),
:deep(.p-password),
:deep(.p-password-input) {
  width: 100% !important;
}

/* Ensure the password input container matches */
:deep(.p-password) {
  display: block;
  width: 100%;
}

/* Additional password field adjustments to match email field */
:deep(.p-password .p-password-input) {
  width: 100% !important;
  box-sizing: border-box;
}

.pi-eye, .pi-eye-slash {
    transform: scale(1.6);
    margin-right: 1rem;
    color: #a5b4fc;
}

/* Glassmorphism effect */
.glow-effect {
    position: relative;
    border-radius: 24px;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(16px);
    border: 1px solid rgba(255, 255, 255, 0.5);
    box-shadow: 
        0 8px 32px rgba(31, 38, 135, 0.1),
        0 0 0 1px rgba(255, 255, 255, 0.8);
    overflow: hidden;
}

/* Neural network background pattern */
.bg-neural-pattern {
    background-image: url("data:image/svg+xml,%3Csvg width='100' height='100' viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M11 18c3.866 0 7-3.134 7-7s-3.134-7-7-7-7 3.134-7 7 3.134 7 7 7zm48 25c3.866 0 7-3.134 7-7s-3.134-7-7-7-7 3.134-7 7 3.134 7 7 7zm-43-7c1.657 0 3-1.343 3-3s-1.343-3-3-3-3 1.343-3 3 1.343 3 3 3zm63 31c1.657 0 3-1.343 3-3s-1.343-3-3-3-3 1.343-3 3 1.343 3 3 3zM34 90c1.657 0 3-1.343 3-3s-1.343-3-3-3-3 1.343-3 3 1.343 3 3 3zm56-76c1.657 0 3-1.343 3-3s-1.343-3-3-3-3 1.343-3 3 1.343 3 3 3zM12 86c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm28-65c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm23-11c2.76 0 5-2.24 5-5s-2.24-5-5-5-5 2.24-5 5 2.24 5 5 5zm-6 60c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm29 22c2.76 0 5-2.24 5-5s-2.24-5-5-5-5 2.24-5 5 2.24 5 5 5zM32 63c2.76 0 5-2.24 5-5s-2.24-5-5-5-5 2.24-5 5 2.24 5 5 5zm57-13c2.76 0 5-2.24 5-5s-2.24-5-5-5-5 2.24-5 5 2.24 5 5 5zm-9-21c1.105 0 2-.895 2-2s-.895-2-2-2-2 .895-2 2 .895 2 2 2zM60 91c1.105 0 2-.895 2-2s-.895-2-2-2-2 .895-2 2 .895 2 2 2zM35 41c1.105 0 2-.895 2-2s-.895-2-2-2-2 .895-2 2 .895 2 2 2zM12 60c1.105 0 2-.895 2-2s-.895-2-2-2-2 .895-2 2 .895 2 2 2z' fill='%23ffffff' fill-opacity='0.1' fill-rule='evenodd'/%3E%3C/svg%3E");
}

/* Custom styling for Input fields */
:deep(.p-inputtext) {
    background: #ffffff !important;
    color: #1f2937 !important;
    border: 1px solid #e5e7eb !important;
    transition: all 0.3s ease;
}

:deep(.p-inputtext:focus) {
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2) !important;
    border-color: #3b82f6 !important;
}

:deep(.p-inputtext::placeholder) {
    color: #9ca3af !important;
}

/* Custom Password component styling */
:deep(.p-password-input) {
    background: #ffffff !important;
    color: #1f2937 !important;
    border: 1px solid #e5e7eb !important;
    width: 100% !important;
    padding: 0.75rem !important;
    border-radius: 0.5rem !important;
}

:deep(.p-password-panel) {
    background: #1e293b !important;
    border: 1px solid rgba(148, 163, 184, 0.2) !important;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1) !important;
}

:deep(.custom-password .p-password-input) {
    width: 100% !important;
}

/* Animation for logo pulsing */
.pulse-effect {
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0% {
        filter: drop-shadow(0 0 0px rgba(79, 70, 229, 0.8));
    }
    50% {
        filter: drop-shadow(0 0 10px rgba(79, 70, 229, 0.8));
    }
    100% {
        filter: drop-shadow(0 0 0px rgba(79, 70, 229, 0.8));
    }
}

.pulse-ring {
    border-radius: 50%;
    animation: pulse-ring 2s cubic-bezier(0.455, 0.03, 0.515, 0.955) infinite;
}

@keyframes pulse-ring {
    0% {
        box-shadow: 0 0 0 0 rgba(124, 58, 237, 0.5);
    }
    70% {
        box-shadow: 0 0 0 15px rgba(124, 58, 237, 0);
    }
    100% {
        box-shadow: 0 0 0 0 rgba(124, 58, 237, 0);
    }
}

/* Particle animation */
.particles-container {
    position: absolute;
    width: 100%;
    height: 100%;
}

.particle {
    position: absolute;
    width: 4px;
    height: 4px;
    background: rgba(99, 102, 241, 0.2);
    border-radius: 50%;
    opacity: 0.2;
    animation: float 15s infinite ease-in-out;
}

.particle:nth-child(odd) {
    width: 6px;
    height: 6px;
    background: rgba(99, 102, 241, 0.3);
}

.particle:nth-child(3n) {
    width: 8px;
    height: 8px;
    background: rgba(139, 92, 246, 0.3);
}

.particle:nth-child(5n) {
    width: 3px;
    height: 3px;
    background: rgba(236, 72, 153, 0.3);
}

@keyframes float {
    0% {
        transform: translate(calc(100vw * var(--i, 0)), calc(100vh * var(--j, 0)));
        opacity: 0;
    }
    10% {
        opacity: 0.5;
    }
    90% {
        opacity: 0.5;
    }
    100% {
        transform: translate(calc(100vw * var(--x, 1)), calc(100vh * var(--y, 1)));
        opacity: 0;
    }
}

/* Generating dynamic positions for particles */
.particle:nth-child(1) { --i: 0.1; --j: 0.2; --x: 0.8; --y: 0.9; animation-delay: 0s; }
.particle:nth-child(2) { --i: 0.9; --j: 0.1; --x: 0.1; --y: 0.8; animation-delay: 0.5s; }
.particle:nth-child(3) { --i: 0.3; --j: 0.9; --x: 0.7; --y: 0.2; animation-delay: 1s; }
.particle:nth-child(4) { --i: 0.8; --j: 0.4; --x: 0.2; --y: 0.6; animation-delay: 1.5s; }
.particle:nth-child(5) { --i: 0.5; --j: 0.1; --x: 0.9; --y: 0.5; animation-delay: 2s; }
.particle:nth-child(6) { --i: 0.2; --j: 0.7; --x: 0.8; --y: 0.3; animation-delay: 2.5s; }
.particle:nth-child(7) { --i: 0.6; --j: 0.6; --x: 0.4; --y: 0.4; animation-delay: 3s; }
.particle:nth-child(8) { --i: 0.3; --j: 0.3; --x: 0.7; --y: 0.7; animation-delay: 3.5s; }
.particle:nth-child(9) { --i: 0.9; --j: 0.5; --x: 0.1; --y: 0.5; animation-delay: 4s; }
.particle:nth-child(10) { --i: 0.4; --j: 0.8; --x: 0.6; --y: 0.2; animation-delay: 4.5s; }
.particle:nth-child(11) { --i: 0.7; --j: 0.2; --x: 0.3; --y: 0.8; animation-delay: 5s; }
.particle:nth-child(12) { --i: 0.2; --j: 0.5; --x: 0.8; --y: 0.5; animation-delay: 5.5s; }
.particle:nth-child(13) { --i: 0.8; --j: 0.3; --x: 0.2; --y: 0.7; animation-delay: 6s; }
.particle:nth-child(14) { --i: 0.5; --j: 0.9; --x: 0.5; --y: 0.1; animation-delay: 6.5s; }
.particle:nth-child(15) { --i: 0.1; --j: 0.6; --x: 0.9; --y: 0.4; animation-delay: 7s; }
.particle:nth-child(16) { --i: 0.6; --j: 0.1; --x: 0.4; --y: 0.9; animation-delay: 7.5s; }
.particle:nth-child(17) { --i: 0.3; --j: 0.4; --x: 0.7; --y: 0.6; animation-delay: 8s; }
.particle:nth-child(18) { --i: 0.9; --j: 0.8; --x: 0.1; --y: 0.2; animation-delay: 8.5s; }
.particle:nth-child(19) { --i: 0.4; --j: 0.2; --x: 0.6; --y: 0.8; animation-delay: 9s; }
.particle:nth-child(20) { --i: 0.7; --j: 0.7; --x: 0.3; --y: 0.3; animation-delay: 9.5s; }
.particle:nth-child(21) { --i: 0.2; --j: 0.9; --x: 0.8; --y: 0.1; animation-delay: 10s; }
.particle:nth-child(22) { --i: 0.8; --j: 0.5; --x: 0.2; --y: 0.5; animation-delay: 10.5s; }
.particle:nth-child(23) { --i: 0.5; --j: 0.3; --x: 0.5; --y: 0.7; animation-delay: 11s; }
.particle:nth-child(24) { --i: 0.1; --j: 0.4; --x: 0.9; --y: 0.6; animation-delay: 11.5s; }
.particle:nth-child(25) { --i: 0.6; --j: 0.9; --x: 0.4; --y: 0.1; animation-delay: 12s; }
.particle:nth-child(26) { --i: 0.3; --j: 0.6; --x: 0.7; --y: 0.4; animation-delay: 12.5s; }
.particle:nth-child(27) { --i: 0.9; --j: 0.2; --x: 0.1; --y: 0.8; animation-delay: 13s; }
.particle:nth-child(28) { --i: 0.4; --j: 0.7; --x: 0.6; --y: 0.3; animation-delay: 13.5s; }
.particle:nth-child(29) { --i: 0.7; --j: 0.4; --x: 0.3; --y: 0.6; animation-delay: 14s; }
.particle:nth-child(30) { --i: 0.2; --j: 0.1; --x: 0.8; --y: 0.9; animation-delay: 14.5s; }
.particle:nth-child(31) { --i: 0.8; --j: 0.6; --x: 0.2; --y: 0.4; animation-delay: 15s; }
.particle:nth-child(32) { --i: 0.5; --j: 0.8; --x: 0.5; --y: 0.2; animation-delay: 15.5s; }
.particle:nth-child(33) { --i: 0.1; --j: 0.5; --x: 0.9; --y: 0.5; animation-delay: 16s; }
.particle:nth-child(34) { --i: 0.6; --j: 0.3; --x: 0.4; --y: 0.7; animation-delay: 16.5s; }
.particle:nth-child(35) { --i: 0.3; --j: 0.7; --x: 0.7; --y: 0.3; animation-delay: 17s; }
.particle:nth-child(36) { --i: 0.9; --j: 0.4; --x: 0.1; --y: 0.6; animation-delay: 17.5s; }
.particle:nth-child(37) { --i: 0.4; --j: 0.1; --x: 0.6; --y: 0.9; animation-delay: 18s; }
.particle:nth-child(38) { --i: 0.7; --j: 0.5; --x: 0.3; --y: 0.5; animation-delay: 18.5s; }
.particle:nth-child(39) { --i: 0.2; --j: 0.8; --x: 0.8; --y: 0.2; animation-delay: 19s; }
.particle:nth-child(40) { --i: 0.8; --j: 0.2; --x: 0.2; --y: 0.8; animation-delay: 19.5s; }

/* Button styling enhancement */
:deep(.p-button) {
    transition: all 0.3s ease !important;
    border-radius: 0.5rem !important;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06) !important;
}

:deep(.p-button:hover) {
    transform: translateY(-2px) !important;
    box-shadow: 0 10px 15px -3px rgba(59, 130, 246, 0.1) !important;
}

:deep(.p-button:active) {
    transform: translateY(0) !important;
}

/* Checkbox styling */
:deep(.p-checkbox) {
    width: 20px !important;
    height: 20px !important;
}

:deep(.p-checkbox .p-checkbox-box) {
    background: #ffffff !important;
    border: 1px solid #e5e7eb !important;
}

:deep(.p-checkbox .p-checkbox-box.p-highlight) {
    background: #3b82f6 !important;
    border-color: #3b82f6 !important;
}

:deep(.p-checkbox .p-checkbox-box .p-checkbox-icon) {
    color: #ffffff !important;
    font-size: 14px !important;
}

/* Responsive adjustments */
@media (max-width: 640px) {
    .glow-effect {
        max-width: 90vw;
    }
    
    :deep(.p-password-panel) {
        width: 90vw !important;
        max-width: 90vw !important;
    }
}

@media (min-width: 641px) and (max-width: 1024px) {
    .glow-effect {
        max-width: 80vw;
    }
}

@media (min-width: 1025px) {
    .glow-effect {
        max-width: 500px;
    }
}

/* Additional input field focus and hover effects */
:deep(.p-inputtext:hover:not(:focus)) {
    border-color: rgba(99, 102, 241, 0.3) !important;
}

:deep(.p-password-input:hover:not(:focus)) {
    border-color: rgba(99, 102, 241, 0.3) !important;
}

/* Enhanced hover effect for links */
.text-blue-600:hover {
    text-shadow: 0 0 8px rgba(59, 130, 246, 0.4);
}

/* Animation for gradient background */
@keyframes gradientShift {
    0% {
        background-position: 0% 50%;
    }
    50% {
        background-position: 100% 50%;
    }
    100% {
        background-position: 0% 50%;
    }
}

.bg-gradient-to-br {
    background-size: 200% 200%;
    animation: gradientShift 15s ease infinite;
}

/* Additional styling for error states */
:deep(.p-invalid) {
    border-color: #ef4444 !important;
}

:deep(.p-error) {
    color: #ef4444 !important;
    margin-top: 0.25rem;
    font-size: 0.875rem;
}
</style>