import {EyesCloseIcon} from "@/components/delivery/icons/eyes-close-icon.tsx";
import {EyesOpenIcon} from "@/components/delivery/icons/eyes-open-icon.tsx";


interface SidebarEyesProps {
  isOpen?: boolean,
  onClick?: (() => void) | undefined,
}

export function SidebarEyes({isOpen, onClick}: SidebarEyesProps) {
  return (
    <>
      {isOpen ?
        <EyesOpenIcon onClick={onClick} /> : <EyesCloseIcon onClick={onClick}/>
      }
    </>
  )
}