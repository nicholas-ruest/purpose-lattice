import { FormEvent, useEffect, useState } from "react";
import { api, Covenant, Evaluation } from "./client";
type View = "evaluate" | "register" | "audit";
export function App() {
  const [items, setItems] = useState<Covenant[]>([]);
  const [selected, setSelected] = useState("support-cases");
  const [result, setResult] = useState<Evaluation | null>(null);
  const [audit, setAudit] = useState<unknown[]>([]);
  const [view, setView] = useState<View>("evaluate");
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState("");
  const [error, setError] = useState("");
  const login = async (role: "steward" | "analyst" = "steward") => {
    await api.session(role);
  };
  const refresh = async () => {
    setBusy(true);
    try {
      await login();
      const rows = await api.list();
      setItems(rows);
      if (rows[0]) setSelected(rows[0].dataset_id);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };
  useEffect(() => {
    void refresh();
  }, []);
  const submit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError("");
    setBusy(true);
    const d = new FormData(e.currentTarget);
    try {
      await login("steward");
      const r = await api.create({
        dataset_id: d.get("id"),
        name: d.get("name"),
        owner: d.get("owner"),
        allowed_purposes: ["quality analytics"],
        sensitive_fields: ["email", "message"],
        allowed_regions: ["ca", "uk"],
        retention_days: 30,
        sensitivity: "confidential",
      });
      setNotice("Registered " + r.name);
      setSelected(r.dataset_id);
      setView("evaluate");
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };
  const evaluate = async (denied = false) => {
    setError("");
    setNotice("");
    setBusy(true);
    try {
      await login("analyst");
      const r = await api.evaluate(selected, {
        dataset_id: selected,
        purpose: denied ? "sell targeted ads" : "service quality analysis",
        selected_fields: denied
          ? ["email", "message"]
          : ["category", "resolution_minutes"],
        region: denied ? "us" : "ca",
        retention_days: denied ? 365 : 7,
        operation: denied ? "export" : "aggregate",
      });
      setResult(r);
      setNotice("Plan allowed and appended to RVF");
    } catch (e) {
      setError(String(e));
      if (denied) setNotice("Policy denial confirmed; no plan was appended");
    } finally {
      setBusy(false);
    }
  };
  const loadAudit = async () => {
    setBusy(true);
    try {
      await login("analyst");
      setAudit(await api.audit(selected));
      setView("audit");
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };
  return (
    <main>
      <header>
        <div>
          <span className="eyebrow">Governed data use</span>
          <h1>Purpose Lattice</h1>
          <p>
            Keep agent data inside its declared purpose, geography and lifetime.
          </p>
        </div>
        <div className="pulse">LIVE · RVF</div>
      </header>
      <nav>
        <button onClick={() => setView("evaluate")}>Evaluate</button>
        <button onClick={() => setView("register")}>New covenant</button>
        <button onClick={loadAudit}>Audit</button>
      </nav>
      {notice && <div className="notice">{notice}</div>}
      {error && (
        <div role="alert" className="error">
          {error}
        </div>
      )}
      {busy && <div className="loading">Evaluating purpose boundary…</div>}
      {view === "register" && (
        <section className="panel">
          <h2>Register a data covenant</h2>
          <form onSubmit={submit}>
            <label>
              Dataset ID
              <input name="id" required defaultValue="returns-cases" />
            </label>
            <label>
              Name
              <input name="name" required defaultValue="Returns case corpus" />
            </label>
            <label>
              Owner
              <input name="owner" required defaultValue="customer-trust" />
            </label>
            <button>Create covenant</button>
          </form>
        </section>
      )}
      {view === "evaluate" && (
        <>
          <section className="grid">
            <article className="panel">
              <h2>Active covenants</h2>
              {items.map((x) => (
                <button
                  key={x.dataset_id}
                  className={selected === x.dataset_id ? "card active" : "card"}
                  onClick={() => setSelected(x.dataset_id)}
                >
                  <strong>{x.name}</strong>
                  <span>{x.allowed_purposes.join(", ")}</span>
                  <small>
                    {x.allowed_regions.join(" · ")} · {x.retention_days} days
                  </small>
                </button>
              ))}
            </article>
            <article className="panel">
              <h2>Plan evaluation</h2>
              <p>
                Apache Arrow materializes field use; conformal bounds measure
                semantic drift; RuVector recalls analogue decisions; RVF seals
                the allowed result.
              </p>
              <button onClick={() => evaluate(false)}>
                Evaluate safe plan
              </button>
              <button className="secondary" onClick={() => evaluate(true)}>
                Try forbidden export
              </button>
              {result && (
                <div className={"result " + result.decision}>
                  <span>{result.decision}</span>
                  <strong>{(result.risk_score * 100).toFixed(1)}%</strong>
                  <small>
                    Risk · {(result.minimization_ratio * 100).toFixed(0)}%
                    minimized · {result.selected_columns} Arrow rows
                  </small>
                  <code>{result.receipt.slice(0, 16)}…</code>
                </div>
              )}
            </article>
          </section>
        </>
      )}
      {view === "audit" && (
        <section className="panel">
          <h2>Restart-durable audit</h2>
          <p>Rehydrated from the append-only RVF archive.</p>
          <pre data-testid="audit-json">{JSON.stringify(audit, null, 2)}</pre>
        </section>
      )}
    </main>
  );
}
