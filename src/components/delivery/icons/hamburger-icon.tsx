interface HamburgerIconProps {
  className?: string;
}

export function HamburgerIcon({ className }: HamburgerIconProps) {
  return (
    <svg
      width="20"
      height="20"
      viewBox="0 0 17 21"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <title>hamburger-icon</title>
      <path
        d="M2.125 10.5H14.875M2.125 15.75H14.875M2.125 5.25H14.875"
        stroke="white"
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
