import { Component, OnInit } from "@angular/core";
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators
} from "@angular/forms";
import { Router, ActivatedRoute } from "@angular/router";
import { Scope, ScopeModelService } from "../scope-model.service";
import { session } from "../model";

@Component({
  selector: "scope-form",
  templateUrl: "./scope-form.component.html",
  styleUrls: ["./scope-form.component.css"]
})
export class ScopeFormComponent implements OnInit {
  scope: FormGroup;

  constructor(
    private fb: FormBuilder,
    private scopeModelService: ScopeModelService,
    private router: Router,
    private route: ActivatedRoute
  ) {
    this.scope = fb.group({
      name: ["", [Validators.required]],
      description: ["", [Validators.required]]
    });
  }

  ngOnInit() {}

  create({ value, valid }: { value: Scope; valid: boolean }) {
    let currUser = session.currUser();
    if (currUser) {
      let applicationId = this.route.parent.snapshot.params["id"];
      this.scopeModelService
        .create(currUser.id, applicationId, value)
        .subscribe((response: any) => {
          this.router.navigate([
            "dashboard/applications",
            applicationId,
            "scopes"
          ]);
        });
    } else {
      this.router.navigate(["login"]);
    }
  }
}
