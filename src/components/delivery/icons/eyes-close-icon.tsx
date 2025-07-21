import { Button } from "@/components/ui/button.tsx";

interface EyesCloseIconProps {
  onClick?: (() => void) | undefined;
}

export function EyesCloseIcon({ onClick }: EyesCloseIconProps) {
  return (
    <Button variant={"link"} onClick={onClick} className={"cursor-pointer"}>
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <title>eyes-close-icon</title>
        <path
          d="M15 18L14.278 14.75M2 8C2.74835 10.0508 4.10913 11.8219 5.8979 13.0733C7.68667 14.3247 9.81695 14.9959 12 14.9959C14.1831 14.9959 16.3133 14.3247 18.1021 13.0733C19.8909 11.8219 21.2516 10.0508 22 8M20 15L18.274 12.95M4 15L5.726 12.95M9 18L9.722 14.75"
          stroke="#383838"
          strokeWidth="1.2"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
    </Button>
  );
}
