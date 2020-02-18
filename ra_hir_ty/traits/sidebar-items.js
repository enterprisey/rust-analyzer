initSidebarItems({"enum":[["AssocTyValue","An associated type value. Usually this comes from a `type` declaration inside an impl block, but for built-in impls we have to synthesize it. (We only need this because Chalk wants a unique ID for each of these.)"],["FnTrait",""],["Guidance","When a goal holds ambiguously (e.g., because there are multiple possible solutions), we issue a set of guidance back to type inference."],["Impl","An impl. Usually this comes from an impl block, but some built-in types get synthetic impls."],["Obligation","Something that needs to be proven (by Chalk) during type checking, e.g. that a certain type implements a certain trait. Proving the Obligation might result in additional information about inference variables."],["Solution","A (possible) solution for a proposed goal."]],"struct":[["AssocTyValueId","This exists just for Chalk, because it needs a unique ID for each associated type value in an impl (even synthetic ones)."],["ClosureFnTraitImplData",""],["GlobalImplId","This exists just for Chalk, because our ImplIds are only unique per module."],["InEnvironment","Something (usually a goal), along with an environment."],["ProjectionPredicate",""],["SolutionVariables",""],["TraitEnvironment","A set of clauses that we assume to be true. E.g. if we are inside this function: `rust fn foo<T: Default>(t: T) {} ` we assume that `T: Default`."],["TraitSolver",""]]});