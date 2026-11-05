<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Profile Derivation and Backend Decision Basis

**Document version:** 0.4.0  
**Status:** Engineering derivation — NOT AN APPROVED NUMERIC PROFILE  
**Scope:** V1 generic non-financial operating company; three historical periods; five forecast periods  
**Reference implementation:** Numeric Envelope POC, branch `research/ofrl-ib-001-numeric-envelope`  
**Decision:** No production arithmetic backend is selected by this document.

## 1. Executive result

The current V1 conventions do not yet define a finite upper bound for several material inputs. Consequently, a finite worst-case numeric envelope cannot be derived from the current contract alone.

The missing upper bounds include:

- Revenue growth above its permitted lower bound of -100%.
- COGS / revenue and operating expenses / revenue.
- DSO, DIO and DPO.
- CapEx / revenue.
- Debt drawdowns.
- Interest rate.

Tax rate, depreciation rate and dividend payout ratio already have a proposed interval of 0 to 1. Principal repayment is constrained by opening debt plus current-period drawdowns. These facts are useful, but do not close the remaining unbounded paths.

This document defines the symbolic derivation required once the Model Owner supplies approved input bounds. It does not choose those bounds, infer them from synthetic Case A, or treat the POC stress values as approved operating limits.

**Gate result: OPEN.** Numeric Profile approval and production backend selection remain blocked pending a finite model domain, formula-chain headroom, error budgets, resource evidence, dependency disposition and independent Finance/Accounting review.

## 2. Model variables and bound notation

All monetary variables below are expressed in model units. Let:

- \(t \in \{1,2,3,4,5\}\) identify forecast years.
- \(R_0\) be last historical revenue.
- \(\overline{R}_0\) be the approved maximum absolute opening revenue.
- \(g_t\) be the period revenue growth assumption.
- \(\overline{g}\) be its approved finite upper bound, with \(g_t \ge -1\).
- \(F = 1 + \overline{g}\) be the maximum annual revenue multiplier.
- \(\overline{c}\), \(\overline{o}\), \(\overline{k}\) be approved upper bounds for COGS / revenue, operating expenses / revenue and CapEx / revenue.
- \(\overline{d}_{AR}\), \(\overline{d}_{INV}\), \(\overline{d}_{AP}\) be approved upper bounds for DSO, DIO and DPO.
- \(\overline{r}_{DEP}\), \(\overline{r}_{INT}\), \(\overline{r}_{TAX}\), \(\overline{r}_{DIV}\) be approved upper bounds for depreciation, interest, tax and dividend payout assumptions.
- \(\overline{U}_t\) be the approved maximum debt drawdown in period \(t\).
- An overbar denotes an approved maximum absolute value, not an observed sample maximum.

Every bound must have a source, owner, unit, rationale and version. A TBD value is an unresolved gate, not zero or unlimited permission.

## 3. Five-year operating envelope

### 3.1 Revenue compounding

The V1 formula is:

\[
R_t = R_{t-1}(1+g_t)
\]

If \(R_0 \ge 0\), \(-1 \le g_t \le \overline{g}\), and \(\overline{g}\) is finite:

\[
\overline{R}_t = \overline{R}_0 F^t,
\qquad F=1+\overline{g}
\]

The five-year maximum is therefore:

\[
\overline{R}_5 = \overline{R}_0 F^5
\]

A lower growth bound alone does not create a finite upper revenue bound. The current lower bound of -100% prevents negative revenue through this formula, but does not constrain positive compounding.

### 3.2 Operating lines and CapEx

Given finite non-negative driver bounds:

\[
\overline{COGS}_t = \overline{R}_t\,\overline{c}
\]

\[
\overline{OPEX}_t = \overline{R}_t\,\overline{o}
\]

\[
\overline{CapEx}_t = \overline{R}_t\,\overline{k}
\]

These bounds are not derivable while the respective upper limits are TBD. The model intentionally permits COGS and operating expense ratios above 100%; loss-making periods are valid. Therefore, an undocumented 100% cap must not be introduced as an engineering shortcut.

## 4. Formula-chain intermediate envelope

The implementation order matters. The POC computes the following multiplication numerators before division by 365. The bound must therefore apply to the product, not only to the smaller final balance.

| Formula node | POC evaluation order | Symbolic maximum absolute intermediate |
|---|---|---|
| Revenue | \(R_{t-1}(1+g_t)\) | \(\overline{R}_{t-1}F\) |
| COGS | \(R_t c_t\) | \(\overline{R}_t\overline{c}\) |
| Operating expenses | \(R_t o_t\) | \(\overline{R}_t\overline{o}\) |
| CapEx | \(R_t k_t\) | \(\overline{R}_t\overline{k}\) |
| AR numerator | \(R_t DSO_t\) | \(\overline{R}_t\overline{d}_{AR}\) |
| Inventory numerator | \(COGS_t DIO_t\) | \(\overline{R}_t\overline{c}\overline{d}_{INV}\) |
| AP numerator | \(COGS_t DPO_t\) | \(\overline{R}_t\overline{c}\overline{d}_{AP}\) |
| AR | AR numerator / 365 | \(\overline{R}_t\overline{d}_{AR}/365\) |
| Inventory | Inventory numerator / 365 | \(\overline{R}_t\overline{c}\overline{d}_{INV}/365\) |
| AP | AP numerator / 365 | \(\overline{R}_t\overline{c}\overline{d}_{AP}/365\) |

The input contract must define whether a decimal day value is permitted to be fractional and set its maximum source scale as well as maximum magnitude. The fact that the final formula divides by 365 does not reduce the intermediate multiplication requirement.

### 4.1 PP&E and depreciation

Let \(G_0\) be opening gross PP&E and \(\overline{G}_0\) its approved bound.

\[
G_t = G_{t-1}+CapEx_t
\]

\[
\overline{G}_t = \overline{G}_0 + \sum_{i=1}^{t}\overline{CapEx}_i
\]

The average gross PP&E numerator is evaluated before division by 2:

\[
N^{PPE}_{AVG,t}=G_{t-1}+G_t
\]

\[
\overline{N}^{PPE}_{AVG,t}
\le \overline{G}_{t-1}+\overline{G}_t
\]

Opening net PP&E is \(G_{t-1}-AccumDep_{t-1}\). Under the V1 validity rule \(0 \le AccumDep_{t-1} \le G_{t-1}\), its absolute value is bounded by \(\overline{G}_{t-1}\).

The two candidate depreciation amounts are:

\[
Dep^{rate}_t = AvgGrossPPE_t\,DepRate_t
\]

\[
Dep^{capacity}_t = OpeningNetPPE_t+CapEx_t
\]

The selected depreciation is their minimum. Its finite bound depends on the finite gross PP&E, CapEx and depreciation-rate bounds. Accumulated depreciation and net PP&E must also be bounded and checked against gross PP&E at every period.

### 4.2 Debt and interest

Let \(D_0\) be opening interest-bearing debt and \(\overline{D}_0\) its approved maximum.

\[
DebtAfterDrawdown_t = D_{t-1}+U_t
\]

\[
D_t = DebtAfterDrawdown_t-Repayment_t
\]

With the existing semantic rule \(0 \le Repayment_t \le DebtAfterDrawdown_t\), and finite drawdown limits:

\[
\overline{D}_t = \overline{D}_0 + \sum_{i=1}^{t}\overline{U}_i
\]

The average debt numerator is:

\[
N^{Debt}_{AVG,t}=D_{t-1}+D_t
\]

\[
\overline{N}^{Debt}_{AVG,t}
\le \overline{D}_{t-1}+\overline{D}_t
\]

Interest is then bounded by:

\[
\overline{Interest}_t
\le
\frac{\overline{D}_{t-1}+\overline{D}_t}{2}
\overline{r}_{INT}
\]

A finite debt drawdown cap and a finite interest-rate cap are both required. Restricting repayments to available debt does not bound debt if drawdowns remain unbounded.

### 4.3 Profit, tax and dividends

The following conservative absolute bounds can be derived by triangle inequalities once the upstream amounts are bounded:

\[
\overline{GrossProfit}_t
\le \overline{R}_t+\overline{COGS}_t
\]

\[
\overline{EBITDA}_t
\le \overline{GrossProfit}_t+\overline{OPEX}_t
\]

\[
\overline{EBIT}_t
\le \overline{EBITDA}_t+\overline{Dep}_t
\]

\[
\overline{EBT}_t
\le \overline{EBIT}_t+\overline{Interest}_t
\]

With \(0 \le TaxRate_t \le \overline{r}_{TAX}\):

\[
\overline{Tax}_t
\le \overline{EBT}_t\overline{r}_{TAX}
\]

\[
\overline{NI}_t
\le \overline{EBT}_t+\overline{Tax}_t
\]

With \(0 \le DividendPayout_t \le \overline{r}_{DIV}\):

\[
\overline{Dividends}_t
\le \overline{NI}_t\overline{r}_{DIV}
\]

These are deliberately conservative envelope bounds; they are not expected values or accounting-materiality thresholds.

### 4.4 Working capital, cash and equity

Let:

\[
\overline{NWC}_t
\le
\overline{AR}_t+
\overline{Inventory}_t+
\overline{AP}_t
\]

\[
\overline{\Delta NWC}_t
\le
\overline{NWC}_t+\overline{NWC}_{t-1}
\]

Then:

\[
\overline{CFO}_t
\le
\overline{NI}_t+
\overline{Dep}_t+
\overline{\Delta NWC}_t
\]

\[
\overline{CFI}_t=\overline{CapEx}_t
\]

The input repayment is constrained by debt available after drawdown. Its absolute upper bound is therefore derived from the prior debt bound and the per-period drawdown bound:

\[
0 \le Repayment_t \le D_{t-1}+U_t
\]

\[
\overline{Repayment}_t
\le \overline{D}_{t-1}+\overline{U}_t
\]

The Cash Flow Statement financing line is evaluated as drawdowns minus repayments minus dividends. A conservative absolute intermediate bound is:

\[
\overline{CFF}_t
\le
\overline{U}_t+
\overline{Repayment}_t+
\overline{Dividends}_t
\]

\[
\overline{\Delta Cash}_t
\le
\overline{CFO}_t+
\overline{CFI}_t+
\overline{CFF}_t
\]

\[
\overline{Cash}_t
\le
\overline{Cash}_{t-1}+\overline{\Delta Cash}_t
\]

The Cash bound is an absolute diagnostic bound. It does not assert that negative closing cash is feasible or reportable. Scenario Feasibility must remain a separate status.

Retained earnings requires its own signed opening bound:

\[
\overline{RE}_t
\le
\overline{RE}_{t-1}+
\overline{NI}_t+
\overline{Dividends}_t
\]

Total asset and total liabilities/equity bounds are obtained by summing the absolute bounds of their constituent accounts. The Balance Sheet residual must be derived separately from arithmetic error propagation; it must not be declared safe merely because exact rational Case A balances to zero.

The Balance Sheet and historical validator also need a finite starting-state envelope. The last historical period is the forecast opening state, but the earlier historical periods must be bounded separately if the validator performs arithmetic on them. Do not derive historical amount maxima solely from forecast driver ratios unless the input contract explicitly constrains the historical data to those same ratios.

### 4.5 Decimal scale and coefficient-growth envelope

An absolute-value bound is not a complete representability proof. Every formula node must carry at least two separate envelopes:

- \(M_x\): maximum absolute numeric value.
- \(S_x\): maximum fractional scale (number of base-10 digits after the decimal point) before any explicitly approved rounding or normalization.

A third measure is needed for arbitrary-precision candidates:

- \(P_x\): maximum coefficient/significant-digit count after normalization.

The following scale rules are conservative *pre-normalization* bounds for exact decimal operations:

| Operation | Magnitude bound | Pre-normalization scale bound |
|---|---|---|
| Addition / subtraction | \(M_x+M_y\) | \(\max(S_x,S_y)\) |
| Multiplication | \(M_xM_y\) | \(S_x+S_y\) |
| Maximum / minimum | \(\max(M_x,M_y)\) | No greater than \(\max(S_x,S_y)\), before backend-specific normalization |
| Division by 2 | \(M_x/2\) | May require one additional fractional digit for an odd final coefficient |
| Division by 365 | \(M_x/365\) | 365 has prime factor 73, so non-terminating decimal results are possible; output scale and rounding are backend/context decisions |

For the five-year revenue recurrence, if the maximum source scale of each annual growth assumption is \(S_g\) and last-historical revenue has maximum scale \(S_{R_0}\), the unreduced multiplication scale can grow as:

\[
S_{R_t} \le S_{R_0}+\sum_{i=1}^{t}S_{g_i}
\]

and therefore:

\[
S_{R_5} \le S_{R_0}+5S_g
\]

This is a scale-growth bound, not a statement that every runtime value will attain that scale. Trailing zeros and common factors can reduce actual scale, but a capacity proof must not depend on sample fixtures happening to contain zeros.

The same operation graph creates these additional scale obligations:

| Formula node | Conservative pre-normalization scale bound |
|---|---|
| \(1+g_t\) | \(S_{g_t}\) |
| Revenue | \(S_{R_{t-1}}+S_{g_t}\) |
| COGS / Opex / CapEx | \(S_{Revenue}+S_{driver}\) |
| AR / inventory / AP numerator | \(S_{revenue\ or\ COGS}+S_{day}\) |
| Gross PP&E | \(\max(S_{opening\ gross\ PPE},S_{CapEx})\) |
| Average gross PP&E numerator | \(\max(S_{opening\ gross\ PPE},S_{closing\ gross\ PPE})\) |
| Average debt numerator | \(\max(S_{opening\ debt},S_{closing\ debt})\) |
| Interest | \(S_{average\ debt}+S_{interest\ rate}\) |
| Tax before floor | \(S_{EBT}+S_{tax\ rate}\) |
| Dividends | \(S_{positive\ NI}+S_{payout}\) |
| NWC / change in NWC | Maximum scale of the participating balances |
| CFO / CFI / CFF / closing cash / retained earnings | Maximum scale of the participating operands, after upstream division policies |

The current input grammar permits up to 28 fractional digits for Decimal strings. That is a lexical/parser ceiling, **not** proof that every combination of 28-digit inputs can pass five years of multiplication without scale reduction, rounding or failure. A profile must set separate source-scale limits for monetary values, ratios/rates and days, then propagate them through the actual formula order. The selected backend must expose any scale reduction or rounding as a characterized arithmetic event; it must never silently convert it into an unobserved acceptance.

For rust_decimal, the candidate's scale is capped at 28 and the mantissa is capped at 96 bits. For each operation, the exact candidate behavior when the nominal product scale exceeds 28 or the scaled coefficient exceeds the mantissa must be tested: it may fail or require a precision-reducing representation step. The profile must prove the chosen path at each boundary, not merely compare the final numeric magnitude.

For BigDecimal, multiplication preserves all coefficient digits rather than automatically limiting every result to the configured division precision. A finite division context therefore does not by itself bound coefficient growth, scale, memory or CPU. The profile must set maximum coefficient digits and scale at material formula nodes and test resource behaviour under the same five-year recurrence.

The Numeric Profile must consequently approve both a *magnitude envelope* and a *scale/coefficient envelope*. A candidate that fits the former but cannot preserve the latter within the approved arithmetic-error budget is not acceptable.

## 4.6 Historical validator arithmetic envelope

The production validator performs arithmetic on the opening Balance Sheet and all three historical periods before a forecast could begin. A finite profile therefore needs separate per-field bounds for historical inputs; forecast driver ratios must not be used to infer them.

Let an overbar denote the approved maximum absolute value of an input or calculated node. These are conservative triangle-inequality bounds, not expected values.

### Opening and historical Balance Sheet

For each opening or historical Balance Sheet:

[
NetPPE = GrossPPE - AccumulatedDepreciation
]

Under the domain condition (0 le AccumulatedDepreciation le GrossPPE):

[
|NetPPE| le overline{GrossPPE}
]

The total-assets calculation is evaluated as the sum of cash, receivables, inventory and net PP&E:

[
overline{TotalAssets}
le
overline{Cash}+
overline{AR}+
overline{Inventory}+
overline{GrossPPE}
]

The liabilities-and-equity calculation is bounded separately:

[
overline{LiabilitiesAndEquity}
le
overline{AP}+
overline{Debt}+
overline{ShareCapital}+
overline{|RetainedEarnings|}
]

Both calculations must fit before their equality comparison. The comparison itself does not add or multiply values, but the two independently calculated totals remain separate evidence nodes.

### Historical Income Statement

The validator evaluates net income sequentially:

[
egin{aligned}
z_1 &= Revenue-COGS\
z_2 &= z_1-Opex\
z_3 &= z_2-Depreciation\
z_4 &= z_3-Interest\
NetIncome &= z_4-Tax
end{aligned}
]

For each subtraction, the conservative intermediate bound is the sum of the absolute bounds of its two operands:

[
overline{|z_1|}leoverline{Revenue}+overline{COGS}
]

[
overline{|z_2|}leoverline{|z_1|}+overline{Opex}
]

The same recurrence applies through depreciation, interest and tax. The final computed bound must be compared with an independently approved absolute bound for reported historical net income. A negative historical net income remains valid; the profile field is a magnitude bound, not a non-negative sign restriction on the actual value.

### Historical NWC and Cash Flow Statement

The NWC formula is evaluated as:

[
NWC = AR+Inventory-AP
]

[
overline{|NWC|}
le
overline{AR}+overline{Inventory}+overline{AP}
]

The period movement is:

[
Delta NWC_t=NWC_t-NWC_{t-1}
]

[
overline{|Delta NWC_t|}
le
overline{|NWC_t|}+overline{|NWC_{t-1}|}
]

The validator then evaluates the Cash Flow Statement in this order:

[
CFO=NI+DepreciationAddback-Delta NWC
]

[
CFI=0-CapEx
]

[
CFF=DebtDrawdowns-PrincipalRepayments-Dividends
]

[
NetChangeCash=CFO+CFI+CFF
]

[
ClosingCash=OpeningCash+NetChangeCash
]

Each sequential addition or subtraction must be assigned an intermediate bound before continuing to the next operation. For example:

[
overline{|CFO|}
le
overline{|NI|}+
overline{|DepreciationAddback|}+
overline{|Delta NWC|}
]

[
overline{|CFF|}
le
overline{DebtDrawdowns}+
overline{PrincipalRepayments}+
overline{Dividends}
]

[
overline{|NetChangeCash|}
le
overline{|CFO|}+overline{|CFI|}+overline{|CFF|}
]

[
overline{|ClosingCash|}
le
overline{|OpeningCash|}+overline{|NetChangeCash|}
]

The opening-cash tie-out, net-income link, depreciation-addback link and closing-cash Balance Sheet tie-out are direct comparisons, not additional arithmetic operations.

### Historical roll-forwards

The remaining checked formulas are:

[
GrossPPE_t=GrossPPE_{t-1}+CapEx_t
]

[
AccumDep_t=AccumDep_{t-1}+Depreciation_t
]

[
DebtAvailable_t=Debt_{t-1}+Drawdowns_t
]

[
Debt_t=DebtAvailable_t-Repayments_t
]

[
RE_t=RE_{t-1}+NI_t-Dividends_t
]

For the arithmetic path, derive the addition intermediate first and then the subtraction intermediate. The domain rule (Repayments_t le DebtAvailable_t) and the PP&E domain rule (AccumDep_t le GrossPPE_t) constrain valid inputs/results, but they do not remove the need to bound the intermediate additions.

A single historical profile section is applied to each of the three historical periods, while opening-balance bounds are kept separate. This is a conservative repeated-period envelope. If the intended historical data domain differs by period, the profile schema should be versioned to express period-specific bounds rather than silently reusing a common limit.

### Division-rounding error propagation

The current V1 forecast path has three non-terminating day-basis divisions per period:

\[
AR_t = \frac{Revenue_t \times DSO_t}{365},\quad
Inventory_t = \frac{COGS_t \times DIO_t}{365},\quad
AP_t = \frac{COGS_t \times DPO_t}{365}
\]

Let \(s_d\) be the selected maximum fractional scale after these divisions, and let \(q=10^{-s_d}\) be one output quantum. Under a nearest rounding mode (\`half_even\` or \`half_up\`), the per-division absolute rounding error is bounded by:

\[
\varepsilon_d \le \frac{q}{2}
\]

Under directed rounding (\`toward_zero\` or \`away_from_zero\`), use the conservative bound:

\[
\varepsilon_d \le q
\]

The candidate-neutral propagation implemented in the tool uses the absolute-value triangle inequality:

\[
E_{NWC,t} \le 3\varepsilon_d
\]

\[
E_{\Delta NWC,1} \le 3\varepsilon_d
\]

\[
E_{\Delta NWC,t} \le E_{NWC,t}+E_{NWC,t-1}
\le 6\varepsilon_d,\quad t\ge 2
\]

Because V1 has no circular cash sweep or cash-dependent operating driver, the conservative closing-cash error accumulates these period CFO errors:

| Forecast period | Closing-cash absolute error bound |
|---|---:|
| Year 1 | \(3\varepsilon_d\) |
| Year 2 | \(9\varepsilon_d\) |
| Year 3 | \(15\varepsilon_d\) |
| Year 4 | \(21\varepsilon_d\) |
| Year 5 | \(27\varepsilon_d\) |

For the Balance Sheet identity, rounded cash, AR, inventory and AP may all contribute. Without assuming error cancellation:

\[
E_{BS,t}
\le
E_{Cash,t}+E_{AR,t}+E_{Inventory,t}+E_{AP,t}
\le 30\varepsilon_d
\]

The derivation compares four separate budgets:

- **Arithmetic approximation:** maximum material forecast-line error, conservatively the five-year closing-cash bound.
- **Accounting identity:** maximum Balance Sheet residual bound.
- **Schedule roll-forward:** maximum NWC/cash schedule value error.
- **Display/export:** decimal-formatting round-off implied by reporting fractional digits and reporting rounding mode.

The derivation assumes non-division additions, subtractions and multiplications are exact within the approved representation envelope, divisions by two use an exact terminating decimal representation, and no circular/feedback dependency causes rounded values to re-enter upstream drivers. A candidate that rounds multiplication results, reduces scale silently, or uses a different division policy requires additional terms or a revised propagation graph.

The display bound covers decimal rounding only. It does **not** cover IEEE-754 conversion into XLSX numeric cells, spreadsheet recalculation, or Excel's 15-significant-digit behavior; those require exporter-specific tests and independent reconciliation. A budget PASS under this formula is conditional on the declared policy and assumptions, not proof that a candidate backend implements them correctly.

### Machine-generated derivation

The candidate-neutral analysis tool is:

- `tools/numeric-envelope/derive.cjs`
- Input contract: [Numeric Profile schema](../../schemas/ofrl-ib-001-numeric-profile-0.1.0.schema.json)
- CI-only fixture: `tools/numeric-envelope/test/fixtures/synthetic-profile.test-only.json`
- Unit tests: `tools/numeric-envelope/test/derive.test.cjs`

The tool uses JavaScript `BigInt` fractions for exact rational bound arithmetic; it does not use binary floating point, add a runtime dependency to the Rust workspace, choose a backend or implement forecast calculations. It enumerates opening/historical validator nodes and the five-year forecast formula graph, records exact rational upper bounds, source/intermediate scale metadata, coefficient-digit evidence where a finite decimal representation is defined, and translates the four separate error-budget inputs to reporting-currency amounts using the declared unit scale.

Division by 365 is always marked context-dependent even when a particular reduced test value happens to cancel factors. If the profile leaves division scale or rounding unresolved, downstream scale/resource checks remain conditional. CPU, allocation, peak-memory, tail-latency and concurrency fields are recorded as target limits only; this derivation tool does not claim to measure them.

Run the derivation with a complete profile JSON:

```bash
node projects/OFRL-IB-001/tools/numeric-envelope/derive.cjs path/to/numeric-profile.json
```

The `--require-approved` option rejects a `test_only` profile. An `approved` input still produces an engineering derivation only; backend adoption and production-engine authorization are separate decisions. The committed synthetic fixture is explicitly test-only and must never be copied into a production approval record or treated as an approved operating domain.

## 5. What the current domain does and does not permit us to conclude

| Input / rule | Current V1 convention | Numeric-envelope consequence |
|---|---|---|
| Revenue growth | Lower bound -100%; no upper bound stated | Five-year revenue upper bound is unbounded |
| COGS / revenue | Non-negative; no 100% cap | COGS and downstream inventory numerator have no finite upper bound |
| Operating expenses / revenue | Non-negative; no upper bound | Operating expense and earnings magnitude have no finite upper bound |
| DSO / DIO / DPO | Non-negative; no upper bound | AR / inventory / AP numerator bounds are unresolved |
| CapEx / revenue | Non-negative; no upper bound | Gross PP&E and depreciation-capacity bounds are unresolved |
| Debt drawdowns | Non-negative; no upper bound | Debt and average-debt bounds are unresolved |
| Interest rate | Non-negative; no upper bound | Interest bound is unresolved |
| Depreciation rate | 0 to 1 | Finite rate factor, conditional on PPE bounds |
| Tax rate | 0 to 1 | Finite rate factor, conditional on EBT bound |
| Dividend payout | 0 to 1 | Finite rate factor, conditional on NI bound |
| Principal repayment | 0 to opening debt plus drawdown | Prevents negative debt, but does not bound debt without drawdown caps |

A finite numerical envelope cannot be certified until the unbounded fields above receive explicit maximum values and the opening-balance classes receive finite absolute bounds. The product's target user and intended company-size range must inform those choices.

## 6. Representation capacity is not the model domain

### 6.1 rust_decimal

The POC evaluates `rust_decimal 1.43.0`, whose candidate representation uses a signed 96-bit mantissa with scale from 0 through 28. For a stored scale \(s\), exact representation requires the magnitude to fit the mantissa after scaling:

\[
|x|\,10^s \le 2^{96}-1
\]

The maximum magnitude therefore depends on the chosen scale. It is incorrect to interpret “28 decimal places” as an unrestricted combination of 28 integer and 28 fractional digits.

The POC helper scales every monetary input by a multiplier of \(10^n\); 10^18 and 10^22 are therefore **input scaling factors**, not the exact maximum output or intermediate magnitudes and not business limits. The stress helper also deliberately replaces assumptions with 100% annual growth, 500% COGS / revenue, 500% operating expenses / revenue, 3650 days for DSO/DIO/DPO, 500% CapEx / revenue, and 100% depreciation, interest, tax and dividend rates.

Using the canonical FY2025 revenue of 1,350 model units, the FY2026 stress revenue is \(2\times1,350\times10^n\). The first AR numerator is therefore:

\[
2\times1,350\times10^n\times3,650
=9,855,000\times10^n
\]

That is \(9.855\times10^{24}\) for the \(10^{18}\) monetary-input multiplier and \(9.855\times10^{28}\) for the \(10^{22}\) multiplier. The latter exceeds the rust_decimal 96-bit maximum mantissa \(79,228,162,514,264,337,593,543,950,335\) at scale zero, so the checked multiplication fails at FY2026 AR numerator. This is a reproducible capacity observation for that exact artificial input combination; it does not locate the full transition boundary or define a supportable company-size limit.

### 6.2 BigDecimal

The POC ran `bigdecimal 0.4.11` with an 80-digit, HalfEven **compile-time default context**. Its 1/3 test produced 80 fractional digits under that build configuration. The POC arithmetic adapter uses BigDecimal operator implementations and verifies the compile-time configuration; it does not demonstrate a run-scoped context passed through the complete forecast graph.

The distinction matters: the crate documentation states that compile-time defaults configure the default Context, while runtime control requires passing explicit Context values to context-aware operations. It also states that multiplication preserves all digits; multiplying two 70-digit operands can produce a 140-digit coefficient, so the default division precision does not cap multiplication growth. See the pinned crate's [BigDecimal 0.4.11 documentation](https://docs.rs/crate/bigdecimal/0.4.11).

This demonstrates a finite division context in the measured POC; it is not a production per-run arithmetic policy. A production adapter must define and test how an immutable run-scoped context reaches **every operation that rounds**, including the exact division path; it must also define a separate maximum coefficient/scale policy for exact multiplication and addition. Context identity, scale policy and rounding mode must be recorded in ModelRun provenance. Arbitrary precision still requires finite input-length, scale, coefficient-digit, CPU and memory limits.

### 6.3 Exact Rational

The POC `BigRational` implementation is a correctness reference. Numerator and denominator size can grow with repeated arithmetic. The POC does not establish an operational resource envelope for using exact rational arithmetic in a production five-year engine; it is not a production candidate decision.

## 7. Error propagation: required method

The current POC establishes exact rational expected values and compares finite-precision outputs against them. It also checks that the candidates have zero display mismatches at a two-decimal test boundary for the tested fixtures. Those are useful observations, but the POC does not establish an approved maximum absolute error or materiality threshold over an approved input domain.

For each arithmetic node \(z=f(x,y)\), maintain an absolute error bound \(E_x,E_y\) relative to the exact rational value and an operation-specific rounding bound \(\epsilon_f\).

Examples:

**Addition / subtraction**

\[
E_{x\pm y}\le E_x+E_y+\epsilon_{add/sub}
\]

**Multiplication**

\[
E_{xy}\le
|x|E_y+|y|E_x+E_xE_y+\epsilon_{mul}
\]

**Division by a fixed positive constant \(d\)**

\[
E_{x/d}\le E_x/d+\epsilon_{div,d}
\]

**Minimum / maximum**

\[
E_{min(x,y)},E_{max(x,y)}
\le \max(E_x,E_y)+\epsilon_{cmp}
\]

The max/min bound uses the fact that min and max are 1-Lipschitz in their operands. The operation-specific error terms must be measured or proven for the chosen backend and context; they must not be assumed to be half an ulp until the candidate's actual rounding behaviour is established.

For each forecast year, propagate these bounds through the same directed formula graph as the implementation, including the prior-period state. A useful formal recurrence is:

\[
\mathbf{E}_t
\le
|J_t|\,\mathbf{E}_{t-1}
+
\boldsymbol{\epsilon}_t
\]

where \(J_t\) is the local sensitivity/Jacobian of the period's recurrence with respect to prior state variables, and \(\boldsymbol{\epsilon}_t\) contains the per-operation error bounds for that period. For the piecewise tax/dividend/depreciation rules, use conservative interval or Lipschitz bounds around branch boundaries.

The result must be calculated over the approved input domain, with worst-case paths and all five years included. Exact rational arithmetic should remain the reference used to verify candidate calculations.

### Required separate budgets

1. **Arithmetic approximation:** finite representation and division error versus exact mathematical formulas.
2. **Accounting identity:** acceptable residual in BS and cross-statement identities, derived from materiality and arithmetic-error contribution.
3. **Schedule roll-forward:** acceptable PP&E, debt, retained-earnings and cash schedule residuals.
4. **Display/export:** rounding and spreadsheet precision differences at the reporting boundary.

Do not combine these into one generic epsilon. Do not widen an error budget after observing a failure merely to produce PASS.

## 8. What the current benchmark evidence proves

The inspected POC evidence includes:

- BM-002: 160 exact expected outputs across five years for the synthetic Case A.
- BM-003: 640 exact expected outputs across four five-year edge scenarios.
- A standard-library Python reference that regenerates the exact rational expected-value files.
- Artificial stress fixtures that multiply all monetary inputs by 10^18 and 10^22 respectively; the 10^22 variant produces an FY2026 AR numerator of 9.855 × 10^28 and triggers the fixed-Decimal overflow. These multipliers are not business limits.
- Non-gating timing observations on a single 4-vCPU hosted runner: BigDecimal approximately 1.50×–2.21× and Exact Rational approximately 7.52×–10.30× the fixed Decimal candidate in the observed cases.

The evidence does **not** establish:
- An approved company-size or input domain.
- A finite worst-case intermediate envelope.
- A model-level arithmetic error budget.
- Memory or allocation behaviour, peak RSS, tail latency or concurrency performance.
- An approved per-run BigDecimal context or production serialization adapter.
- Accounting policy acceptance or independent Finance/Accounting sign-off.
- A selected production arithmetic backend.

The POC comparison function computes exact maximum absolute differences and counts rounded display mismatches. The tests currently assert the number of compared metrics and zero two-decimal display mismatches on selected cases; they do not assert a financially approved maximum error bound. A zero display-mismatch result is not proof that every unrounded difference is immaterial.

## 9. Backend decision protocol

Do not choose a backend using maximum representable value or a single latency table. After the Numeric Profile is approved, evaluate each candidate against the same frozen profile:

| Decision dimension | Required evidence |
|---|---|
| Input representation | Canonical grammar, source precision, exact parse/format round trip |
| Supported domain | Every input and intermediate magnitude **and scale/coefficient** bound fits, with documented capacity headroom |
| Arithmetic semantics | Add/subtract/multiply/divide behaviour, exactness and rounding at every operation |
| Five-year error | Exact-rational comparison across boundaries, edge scenarios and worst-case propagation |
| Context ownership | Explicit, immutable, run-scoped precision and rounding for all rounded operations; separately bound coefficient/scale growth; context ID in provenance |
| Failure semantics | Typed overflow, scale, zero-division and resource-limit failures; no saturation or silent clamping |
| Serialization | Canonical decimal strings and deterministic round-trip |
| Resource use | Controlled latency, allocations, peak memory, tail latency, scale/coefficient growth and concurrency under the approved workload |
| Dependency and migration | Exact versions, source, licences/notices, advisories, MSRV/platforms, lockfiles and cost to migrate the current validator |
| Operational determinism | Repeated clean-process and concurrent-run results with identical canonical input/policy/build |

The production API should hide the chosen arithmetic representation behind a small, typed numeric boundary and typed calculation errors. Do not port the POC's generic JSON-value forecast routine or its stringly typed error surface into the production crate.

## 10. Required next evidence

1. Model Owner supplies a finite intended input domain or a documented distribution plus hard admission limits for every unbounded input class.
2. Convert that domain into per-field magnitude bounds, source-scale bounds, coefficient-digit bounds and five-year intermediate bounds using the derivations in Sections 3–4. Include historical validator arithmetic as well as forecast calculation paths.
3. Confirm lower/upper endpoints, sign rules, source integer/fractional digits, minimum material amount and model unit-scale conversion.
4. Finance/Accounting reviewer confirms formula conventions, materiality and the treatment of negative cash/funding shortfall.
5. Engineering derives arithmetic and scale-propagation error budgets and runs exact-rational boundary tests for every formula chain, including scale just below, at and above the candidate's representable limit.
6. Measure candidate CPU, allocation, peak-memory, tail-latency and concurrency behaviour in a controlled environment.
7. Complete source-level dependency, licence/notice and advisory disposition.
8. Record the approved Numeric Profile, backend, context, errors, limitations, test commit and reviewer sign-offs in a versioned decision record.

Until those requirements are satisfied, the production numeric backend and Production Forecast Engine remain **NOT APPROVED**.
