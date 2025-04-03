<template>
    <div
        id="hero"
        class="flex flex-col pt-6 px-6 lg:px-20 overflow-hidden"
        style="
            background-image: url('/demo/images/landing/AI1.webp');
            background-size: cover;
            background-position: center;
        "
    >
        <div class="mx-6 md:mx-20 mt-0 md:mt-6 bg-white/80 p-6 rounded-lg">
            <h1 class="text-6xl font-bold text-gray-900 leading-tight">
                <span class="font-light block">AI security in ICP</span>
            </h1>
            <p class="font-normal text-2xl leading-normal md:mt-4 text-gray-700">
                AI in security transforms the way we defend against cyber threats, enabling systems to predict, detect, and respond to attacks faster and more accurately than ever before, creating a future where security is proactive, not reactive.
            </p>
            <Button
                label="Get Started"
                style="background-color: #2BBBCE;"
                as="router-link"
                to="/"
                rounded
                class="!text-xl mt-8 !px-4"
            ></Button>
        </div>
        <div class="flex justify-center md:justify-end">
            <img src="/demo/images/landing/screen-1.png" alt="Hero Image" class="w-9/12 md:w-auto" />
        </div>
    </div>
</template>

