import './ui.css'

// Un chiffre clé : "Cette semaine" / "13h31" / "/ 35h"
export default function Stat({ label, value, unit, detail }) {
  return (
    <div className="stat">
      <p className="stat-label">{label}</p>
      <p className="stat-value">
        {value} {unit && <span className="stat-unit">{unit}</span>}
      </p>
      {detail && <p className="stat-detail">{detail}</p>}
    </div>
  )
}
