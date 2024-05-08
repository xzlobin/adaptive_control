---
title: Welcome
layout: default
---

## Formulácia problému

Je potrebné navrhnúť adaptívny riadiaci systém pre riadenie kurzu nákladnej lode, ktorého základom je Lyapunovova teória stability. 

Systém:

$$
\dddot{\varphi}(t)+\left(\frac{\tau_{1}+\tau_{2}}{\tau_{1}\tau_{2}}\right)\ddot{\varphi}(t)+\left(\frac{1}{\tau_{1}\tau_{2}}\right)\dot{\varphi}(t)=\frac{K}{\tau_{1}\tau_{2}}\left(\tau_{3}\dot{u}(t)+u(t)\right),
\label{main_sys}
$$

kde $\varphi(t)$ [rad] je uhol natočenia lode a táto veličina je výstupnou
veličinou riadeného systému. Vstupom je uhol vychýlenia kormidla $u(t)$ [rad]. 
Referenčný model pre URO je v tvare

$$
W_{m}(s)=\frac{k_{m}\cdot Z_{m}(s)}{R_{m}(s)}=\frac{\overbrace{\frac{1}{T_{m}^{2}}}^{k_{m}}\cdot\overbrace{1}^{Z_{m}(s)}}{\underbrace{\left(s+\frac{1}{T_{m}}\right)^{2}}_{R_{m}(s)}},
$$

kde $k_m$ je vysokofrekvenčné zosilnenie referenčného modelu, $Z_m(s)$ je
monický Hurwitzov polynóm stupňa 0, $R_m(s)$ je monický Hurwitzov polynóm stupňa
2, pričom relatívny stupeň $n^\star_m=2$.

## Riešenie

Možno odvodiť prenosovú funkciu systému $\eqref{main_sys}$

$$
\frac{\varphi(s)}{u(s)}=W_p(s)=\frac{\overbrace{\left.\frac{K}{\tau_{1}\tau_{2}\tau_{3}}\right.}^{k_{p}}\overbrace{\left(s+\frac{1}{\tau_{3}}\right)}^{Z_{p}(s)}}{\underbrace{s^{3}+\frac{\tau_{1}+\tau_{2}}{\tau_{1}\tau_{2}}s^{2}+\frac{1}{\tau_{1}\tau_{2}}}_{R_{p}(s)}}=\frac{k_{p}\cdot Z_{p}(s)}{R_{p}(s)}.
$$

Podobne ako pre prenosovú funkciu referenčného modelu, $k_p$ je vysokofrekvenčné zosilnenie, $Z_p(s)$ je
monický Hurwitzov polynóm stupňa 1, $R_p(s)$ je monický Hurwitzov polynóm stupňa
$n=3$, pričom relatívny stupeň stále $n^\star=2$.

Všeobecný tvar zákona riadenia, ktorý rieši MRC problém je

$$
u=\Theta_{1}^{\star\top}\frac{\alpha(s)}{\Lambda(s)}u+\Theta_{2}^{\star\top}\frac{\alpha(s)}{\Lambda(s)}\varphi+\Theta_{3}^{\star}\varphi+\Theta_{4}^{\star}r,
$$

$$
\alpha=\begin{bmatrix}s^{n-2} & \ldots & s & 1\end{bmatrix}^{\top},\qquad \Lambda(s)=\Lambda_{0}(s)Z_{m}(s),\qquad n=3,
$$

$$
\Theta_{1}^{\star\top}=\begin{bmatrix}\Theta_{11}^{\star} & \Theta_{12}^{\star}\end{bmatrix},\qquad\Theta_{2}^{\star\top}=\begin{bmatrix}\Theta_{21}^{\star} & \Theta_{22}^{\star}\end{bmatrix}.
$$

$\Lambda(s)$ je ľubovoľný monický Hurwitzov polynóm stupňa $n - 1 = 2$ obsahujúci $Z_m(s)$ ako faktor. Budeme používať

$$
\Lambda(s)=\left(s+\lambda\right)^{2},\qquad\lambda>0,
$$

riešenie MRC problému je definované podmienkami zhody [1]