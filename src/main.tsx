import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
// import {DevSupport} from "@react-buddy/ide-toolbox";
// import {ComponentPreviews, useInitial} from "@/dev";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
