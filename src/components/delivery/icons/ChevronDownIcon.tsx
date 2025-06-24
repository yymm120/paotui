

interface ChevronDownIconProps {
  className?: string;
  stroke?: string;
}

export function ChevronDownIcon({
  className,
  stroke = "white",
}: ChevronDownIconProps) {
  return (
    <svg
      width="12"
      height="12"
      viewBox="0 0 12 12"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <path
        d="M3 4.5L6 7.5L9 4.5"
        stroke={stroke}
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
