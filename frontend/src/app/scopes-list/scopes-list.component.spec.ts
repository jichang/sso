import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ScopesListComponent } from './scopes-list.component';

describe('ScopesListComponent', () => {
  let component: ScopesListComponent;
  let fixture: ComponentFixture<ScopesListComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ScopesListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ScopesListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
