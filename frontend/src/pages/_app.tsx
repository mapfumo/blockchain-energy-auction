import type { AppProps } from "next/app";
import Head from "next/head";
import { ThemeProvider } from "../contexts/ThemeContext";
import ErrorBoundary from "../components/ErrorBoundary";
import "../styles/globals.css";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ErrorBoundary>
      <ThemeProvider>
        <Head>
          <title>🔋 Energy Trading Dashboard - Australia</title>
          <meta
            name="description"
            content="Real-time energy auction monitoring for Australian solar battery trading"
          />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="icon" href="/favicon.ico?v=2" />
          <link rel="apple-touch-icon" href="/favicon.ico?v=2" />
        </Head>
        <Component {...pageProps} />
      </ThemeProvider>
    </ErrorBoundary>
  );
}
