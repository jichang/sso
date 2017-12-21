import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ApplicationScopesComponent } from './application-scopes.component';

describe('ApplicationScopesComponent', () => {
  let component: ApplicationScopesComponent;
  let fixture: ComponentFixture<ApplicationScopesComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ApplicationScopesComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ApplicationScopesComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
