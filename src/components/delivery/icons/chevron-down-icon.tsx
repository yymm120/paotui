interface ChevronDownIconProps {
  className?: string;
  stroke?: string;
  strokeWidth?: number | string;
}

export function ChevronDownIcon({
  className,
  stroke = "white",
  strokeWidth = 1.5,
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
      <title>chevron-down-icon</title>
      <path
        d="M3 4.5L6 7.5L9 4.5"
        stroke={stroke}
        strokeWidth={strokeWidth}
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
