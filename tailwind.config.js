const config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic": "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
      animation: {
        "infinite-scroll": "infinite-scroll 7.5s linear infinite",
        flashing: "flashing 2s ease-in-out infinite",
      },
      keyframes: {
        "infinite-scroll": {
          from: { transform: "translateX(100%)" },
          to: { transform: "translateX(-100%)" },
        },
        flashing: {
          "0%, 100%": { backgroundColor: "white" },
          "50%": { backgroundColor: "#fcd34d" },
        },
      },
      spacing: {
        14.5: "58px",
        21: "104px",
        29: "116px",
        62: "248px",
      },
    },
  },
  plugins: [],
};
export default config;
